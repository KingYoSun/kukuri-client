import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '@/hooks/use-toast';
import { User } from './auth-store';

interface ProfileState {
  profiles: Record<string, User>;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchProfile: (userId: string) => Promise<User | null>;
  updateProfile: (userId: string, data: { displayName?: string; bio?: string; avatar?: string }) => Promise<boolean>;
  followUser: (userId: string, targetUserId: string) => Promise<boolean>;
  unfollowUser: (userId: string, targetUserId: string) => Promise<boolean>;
}

export const useProfileStore = create<ProfileState>((set, get) => ({
  profiles: {},
  isLoading: false,
  error: null,

  fetchProfile: async (userId: string): Promise<User | null> => {
    // Check if we already have this profile cached
    const cachedProfile = get().profiles[userId];
    if (cachedProfile) {
      return cachedProfile;
    }

    set({ isLoading: true, error: null });
    try {
      const profile: User | null = await invoke('get_profile', { userId });
      
      if (profile) {
        set(state => ({ 
          profiles: { ...state.profiles, [userId]: profile },
          isLoading: false 
        }));
        return profile;
      } else {
        set({ isLoading: false });
        return null;
      }
    } catch (error) {
      console.error('Error fetching profile:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch profile',
      });
      return null;
    }
  },

  updateProfile: async (userId: string, data: { displayName?: string; bio?: string; avatar?: string }): Promise<boolean> => {
    set({ isLoading: true, error: null });
    try {
      const result: { success: boolean; message?: string } = await invoke('update_profile', {
        userId,
        ...data
      });

      if (result.success) {
        // Refresh the profile
        await get().fetchProfile(userId);
        set({ isLoading: false });
        toast({
          title: 'Profile updated',
          description: 'Your profile has been updated successfully',
        });
        return true;
      } else {
        throw new Error(result.message || 'Failed to update profile');
      }
    } catch (error) {
      console.error('Error updating profile:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to update profile',
      });
      return false;
    }
  },

  followUser: async (userId: string, targetUserId: string): Promise<boolean> => {
    set({ isLoading: true, error: null });
    try {
      const result: { success: boolean; message?: string } = await invoke('follow_user', {
        userId,
        targetUserId,
      });

      if (result.success) {
        // Refresh both profiles
        await get().fetchProfile(userId);
        await get().fetchProfile(targetUserId);
        set({ isLoading: false });
        toast({
          title: 'User followed',
          description: 'You are now following this user',
        });
        return true;
      } else {
        throw new Error(result.message || 'Failed to follow user');
      }
    } catch (error) {
      console.error('Error following user:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to follow user',
      });
      return false;
    }
  },

  unfollowUser: async (userId: string, targetUserId: string): Promise<boolean> => {
    set({ isLoading: true, error: null });
    try {
      const result: { success: boolean; message?: string } = await invoke('unfollow_user', {
        userId,
        targetUserId,
      });

      if (result.success) {
        // Refresh both profiles
        await get().fetchProfile(userId);
        await get().fetchProfile(targetUserId);
        set({ isLoading: false });
        toast({
          title: 'User unfollowed',
          description: 'You are no longer following this user',
        });
        return true;
      } else {
        throw new Error(result.message || 'Failed to unfollow user');
      }
    } catch (error) {
      console.error('Error unfollowing user:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to unfollow user',
      });
      return false;
    }
  },
}));