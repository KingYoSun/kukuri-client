import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '@/hooks/use-toast';

export interface User {
  id: string;
  displayName: string;
  bio: string;
  avatar?: string;
  following: string[];
  followers: string[];
  createdAt: number;
}

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  createUser: (displayName: string, bio?: string) => Promise<void>;
  signIn: (userId: string) => Promise<void>;
  logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  isAuthenticated: false,
  isLoading: false,
  error: null,

  createUser: async (displayName: string, bio?: string) => {
    set({ isLoading: true, error: null });
    try {
      const result: { userId: string; success: boolean; message?: string } = await invoke('create_user', {
        displayName,
        bio,
      });

      if (result.success) {
        // Fetch the user profile
        const user: User | null = await invoke('get_profile', { userId: result.userId });
        
        if (user) {
          set({ user, isAuthenticated: true, isLoading: false });
          toast({
            title: 'Account created',
            description: `Welcome, ${user.displayName}!`,
          });
        } else {
          throw new Error('Failed to fetch user profile after creation');
        }
      } else {
        throw new Error(result.message || 'Failed to create user');
      }
    } catch (error) {
      console.error('Error creating user:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to create account',
      });
    }
  },

  signIn: async (userId: string) => {
    set({ isLoading: true, error: null });
    try {
      const result: { userId: string; success: boolean; message?: string } = await invoke('sign_in', {
        userId,
      });

      if (result.success) {
        // Fetch the user profile
        const user: User | null = await invoke('get_profile', { userId: result.userId });
        
        if (user) {
          set({ user, isAuthenticated: true, isLoading: false });
          toast({
            title: 'Signed in',
            description: `Welcome back, ${user.displayName}!`,
          });
        } else {
          throw new Error('Failed to fetch user profile after sign in');
        }
      } else {
        throw new Error(result.message || 'Failed to sign in');
      }
    } catch (error) {
      console.error('Error signing in:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to sign in',
      });
    }
  },

  logout: () => {
    set({ user: null, isAuthenticated: false });
    toast({
      title: 'Signed out',
      description: 'You have been signed out successfully',
    });
  },
}));