import { useCallback } from 'react';
import { usePostStore } from '@/stores/post-store';
import { useAuthStore } from '@/stores/auth-store';
import { CreatePostInput, Post, SearchPostsInput } from '@/models/post';
import { createPost, getPosts, getUserPosts, searchPosts } from '@/services/post-service';
import { toast } from '@/hooks/use-toast';
import { DEFAULT_PAGE_SIZE } from '@/lib/constants';

/**
 * 投稿フック
 * 
 * 投稿の作成、取得、検索に関連する機能を提供します。
 */
export function usePosts() {
  const { 
    posts, 
    userPosts, 
    isLoading, 
    error,
    fetchPosts: storeFetchPosts,
    fetchUserPosts: storeFetchUserPosts,
    createPost: storeCreatePost,
    searchPosts: storeSearchPosts
  } = usePostStore();

  const { user } = useAuthStore();

  /**
   * 投稿を取得します
   */
  const fetchPosts = useCallback(async (limit = DEFAULT_PAGE_SIZE, offset = 0) => {
    try {
      await storeFetchPosts(limit, offset);
      return true;
    } catch (error) {
      console.error('Error fetching posts:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch posts',
      });
      return false;
    }
  }, [storeFetchPosts]);

  /**
   * 特定のユーザーの投稿を取得します
   */
  const fetchUserPosts = useCallback(async (userId: string, limit = DEFAULT_PAGE_SIZE, offset = 0) => {
    try {
      await storeFetchUserPosts(userId, limit, offset);
      return true;
    } catch (error) {
      console.error('Error fetching user posts:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch user posts',
      });
      return false;
    }
  }, [storeFetchUserPosts]);

  /**
   * 新しい投稿を作成します
   */
  const handleCreatePost = useCallback(async (input: CreatePostInput) => {
    if (!user) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'You must be logged in to create a post',
      });
      return false;
    }

    try {
      const result = await createPost(user.id, input);
      if (result.success) {
        await storeCreatePost(input.content);
        return true;
      } else {
        toast({
          variant: 'destructive',
          title: 'Error',
          description: result.message || 'Failed to create post',
        });
        return false;
      }
    } catch (error) {
      console.error('Error creating post:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to create post',
      });
      return false;
    }
  }, [user, storeCreatePost]);

  /**
   * 投稿を検索します
   */
  const handleSearchPosts = useCallback(async (input: SearchPostsInput): Promise<Post[]> => {
    try {
      return await storeSearchPosts(input.query, input.limit);
    } catch (error) {
      console.error('Error searching posts:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to search posts',
      });
      return [];
    }
  }, [storeSearchPosts]);

  /**
   * 現在のユーザーの投稿を取得します
   */
  const fetchCurrentUserPosts = useCallback(async (limit = DEFAULT_PAGE_SIZE, offset = 0) => {
    if (!user) {
      return false;
    }
    return fetchUserPosts(user.id, limit, offset);
  }, [user, fetchUserPosts]);

  return {
    posts,
    userPosts,
    isLoading,
    error,
    fetchPosts,
    fetchUserPosts,
    fetchCurrentUserPosts,
    createPost: handleCreatePost,
    searchPosts: handleSearchPosts,
  };
}