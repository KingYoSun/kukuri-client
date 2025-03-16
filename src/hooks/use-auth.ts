import { useCallback } from 'react';
import { useAuthStore } from '@/stores/auth-store';
import { CreateUserInput, User } from '@/models/user';
import { createUser, signIn, listUsers } from '@/services/auth-service';
import { toast } from '@/hooks/use-toast';

/**
 * 認証フック
 * 
 * ユーザー認証に関連する機能を提供します。
 */
export function useAuth() {
  const { 
    user, 
    isAuthenticated, 
    isLoading, 
    error,
    createUser: storeCreateUser,
    signIn: storeSignIn,
    logout: storeLogout
  } = useAuthStore();

  /**
   * 新しいユーザーを作成します
   */
  const handleCreateUser = useCallback(async (input: CreateUserInput) => {
    try {
      const result = await createUser(input);
      if (result.success) {
        await storeCreateUser(input.displayName, input.bio);
        return true;
      } else {
        toast({
          variant: 'destructive',
          title: 'Error',
          description: result.message || 'Failed to create user',
        });
        return false;
      }
    } catch (error) {
      console.error('Error creating user:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to create user',
      });
      return false;
    }
  }, [storeCreateUser]);

  /**
   * 既存のユーザーでサインインします
   */
  const handleSignIn = useCallback(async (userId: string) => {
    try {
      const result = await signIn(userId);
      if (result.success) {
        await storeSignIn(userId);
        return true;
      } else {
        toast({
          variant: 'destructive',
          title: 'Error',
          description: result.message || 'Failed to sign in',
        });
        return false;
      }
    } catch (error) {
      console.error('Error signing in:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to sign in',
      });
      return false;
    }
  }, [storeSignIn]);

  /**
   * ログアウトします
   */
  const handleLogout = useCallback(() => {
    storeLogout();
  }, [storeLogout]);

  /**
   * 利用可能なすべてのユーザーのリストを取得します
   */
  const getAvailableUsers = useCallback(async () => {
    try {
      return await listUsers();
    } catch (error) {
      console.error('Error listing users:', error);
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to list users',
      });
      return [];
    }
  }, []);

  return {
    user,
    isAuthenticated,
    isLoading,
    error,
    createUser: handleCreateUser,
    signIn: handleSignIn,
    logout: handleLogout,
    getAvailableUsers,
  };
}