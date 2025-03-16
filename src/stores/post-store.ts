import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '@/hooks/use-toast';

export interface Post {
  id: string;
  authorId: string;
  content: string;
  attachments: string[];
  mentions: string[];
  hashtags: string[];
  createdAt: number;
}

interface PostState {
  posts: Post[];
  userPosts: Record<string, Post[]>;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchPosts: (limit?: number, offset?: number) => Promise<void>;
  fetchUserPosts: (userId: string, limit?: number, offset?: number) => Promise<void>;
  createPost: (content: string) => Promise<void>;
  searchPosts: (query: string, limit?: number) => Promise<Post[]>;
}

export const usePostStore = create<PostState>((set, get) => ({
  posts: [],
  userPosts: {},
  isLoading: false,
  error: null,

  fetchPosts: async (limit = 20, offset = 0) => {
    set({ isLoading: true, error: null });
    try {
      const posts: Post[] = await invoke('get_posts', { limit, offset });
      set({ posts, isLoading: false });
    } catch (error) {
      console.error('Error fetching posts:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch posts',
      });
    }
  },

  fetchUserPosts: async (userId: string, limit = 20, offset = 0) => {
    set({ isLoading: true, error: null });
    try {
      const posts: Post[] = await invoke('get_user_posts', { userId, limit, offset });
      set(state => ({ 
        userPosts: { ...state.userPosts, [userId]: posts },
        isLoading: false 
      }));
    } catch (error) {
      console.error('Error fetching user posts:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch user posts',
      });
    }
  },

  createPost: async (content: string) => {
    set({ isLoading: true, error: null });
    try {
      // Get the current user ID from auth store
      const userId = window.__TAURI__?.auth?.userId;
      if (!userId) {
        throw new Error('User not authenticated');
      }

      const result: { post_id: string; success: boolean; message?: string } = await invoke('create_post', {
        authorId: userId,
        content,
      });

      if (result.success) {
        // Refresh the posts
        await get().fetchPosts();
        set({ isLoading: false });
        toast({
          title: 'Post created',
          description: 'Your post has been published successfully',
        });
      } else {
        throw new Error(result.message || 'Failed to create post');
      }
    } catch (error) {
      console.error('Error creating post:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to create post',
      });
    }
  },

  searchPosts: async (query: string, limit = 50): Promise<Post[]> => {
    set({ isLoading: true, error: null });
    try {
      const posts: Post[] = await invoke('search_posts', { query, limit });
      set({ isLoading: false });
      return posts;
    } catch (error) {
      console.error('Error searching posts:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to search posts',
      });
      return [];
    }
  },
}));

// Add a global type for Tauri window
declare global {
  interface Window {
    __TAURI__?: {
      auth?: {
        userId: string;
      };
    };
  }
}