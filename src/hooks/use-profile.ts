import { useCallback } from 'react';
import { useProfileStore } from '@/stores/profile-store';
import { useAuthStore } from '@/stores/auth-store';
import { UpdateProfileInput, User } from '@/models/user';
import { getProfile, updateProfile, followUser, unfollowUser } from '@/services/profile-service';
import { toast } from '@/hooks/use-toast';

/**
 * プロフィールフック
 * 
 * ユーザープロフィールの更新、フォロー関連の機能を提供します。
 */
export function useProfile() {
  const { 
    profiles, 
    isLoading, 
    error,
    fetchProfile: storeFetchProfile,
    updateProfile: storeUpdateProfile,
    followUser: storeFollowUser,
    unfollowUser: storeUnfollowUser
  } = useProfileStore();

  const { user } = useAuthStore();

  /**
   * プロフィールを取得します
   */
  const fetchProfile = useCallback(async (userId: string): Promise<User | null> => {
    try {
      return await storeFetchProfile(userId);
    } catch (error) {
      console.error('Error fetching profile:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch profile',
      });
      return null;
    }
  }, [storeFetchProfile]);

  /**
   * プロフィールを更新します
   */
  const handleUpdateProfile = useCallback(async (userId: string, input: UpdateProfileInput): Promise<boolean> => {
    try {
      return await storeUpdateProfile(userId, input);
    } catch (error) {
      console.error('Error updating profile:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to update profile',
      });
      return false;
    }
  }, [storeUpdateProfile]);

  /**
   * 現在のユーザーのプロフィールを更新します
   */
  const updateCurrentProfile = useCallback(async (input: UpdateProfileInput): Promise<boolean> => {
    if (!user) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'You must be logged in to update your profile',
      });
      return false;
    }
    return handleUpdateProfile(user.id, input);
  }, [user, handleUpdateProfile]);

  /**
   * ユーザーをフォローします
   */
  const handleFollowUser = useCallback(async (targetUserId: string): Promise<boolean> => {
    if (!user) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'You must be logged in to follow users',
      });
      return false;
    }

    if (user.id === targetUserId) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'You cannot follow yourself',
      });
      return false;
    }

    try {
      return await storeFollowUser(user.id, targetUserId);
    } catch (error) {
      console.error('Error following user:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to follow user',
      });
      return false;
    }
  }, [user, storeFollowUser]);

  /**
   * ユーザーのフォローを解除します
   */
  const handleUnfollowUser = useCallback(async (targetUserId: string): Promise<boolean> => {
    if (!user) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'You must be logged in to unfollow users',
      });
      return false;
    }

    try {
      return await storeUnfollowUser(user.id, targetUserId);
    } catch (error) {
      console.error('Error unfollowing user:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to unfollow user',
      });
      return false;
    }
  }, [user, storeUnfollowUser]);

  /**
   * ユーザーがフォローされているかどうかを確認します
   */
  const isFollowing = useCallback((targetUserId: string): boolean => {
    if (!user) return false;
    return user.following.includes(targetUserId);
  }, [user]);

  return {
    profiles,
    isLoading,
    error,
    fetchProfile,
    updateProfile: handleUpdateProfile,
    updateCurrentProfile,
    followUser: handleFollowUser,
    unfollowUser: handleUnfollowUser,
    isFollowing,
  };
}