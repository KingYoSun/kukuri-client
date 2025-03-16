import { describe, it, expect, beforeEach, vi } from 'vitest';
import { useAuthStore } from '@/stores/auth-store';

// Tauriのinvokeをモック化
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

// toastをモック化
vi.mock('@/hooks/use-toast', () => ({
  toast: vi.fn(),
}));

describe('Authentication Flow Integration', () => {
  // テスト用のユーザーデータ
  const testUser = {
    id: '123e4567-e89b-12d3-a456-426614174000',
    displayName: 'Test User',
    bio: 'Test bio',
    avatar: null,
    following: [],
    followers: [],
    createdAt: Date.now(),
  };

  // テスト前にモックをリセットし、ストアを初期化
  beforeEach(() => {
    vi.clearAllMocks();
    
    // Zustandストアを初期化
    const { getState, setState } = useAuthStore;
    setState({
      user: null,
      isAuthenticated: false,
      isLoading: false,
      error: null
    });
  });

  describe('User Creation and Authentication', () => {
    it('should create a new user and authenticate', async () => {
      // createUserのモック
      (invoke as any).mockResolvedValueOnce({
        userId: testUser.id,
        success: true,
      });

      // getProfileのモック
      (invoke as any).mockResolvedValueOnce(testUser);

      // ストアの状態を確認
      const { getState } = useAuthStore;
      const store = getState();
      
      // ユーザー作成
      await store.createUser(testUser.displayName, testUser.bio);

      // 更新された状態を取得
      const updatedStore = getState();
      
      // ストアの状態を確認
      expect(updatedStore.isAuthenticated).toBe(true);
      expect(updatedStore.user).toEqual(testUser);
      expect(invoke).toHaveBeenCalledWith('create_user', {
        displayName: testUser.displayName,
        bio: testUser.bio,
      });
    });

    it('should sign in an existing user', async () => {
      // signInのモック
      (invoke as any).mockResolvedValueOnce({
        userId: testUser.id,
        success: true,
      });

      // getProfileのモック
      (invoke as any).mockResolvedValueOnce(testUser);

      // ストアの状態を確認
      const { getState } = useAuthStore;
      const store = getState();
      
      // サインイン
      await store.signIn(testUser.id);

      // 更新された状態を取得
      const updatedStore = getState();
      
      // ストアの状態を確認
      expect(updatedStore.isAuthenticated).toBe(true);
      expect(updatedStore.user).toEqual(testUser);
      expect(invoke).toHaveBeenCalledWith('sign_in', {
        userId: testUser.id,
      });
    });

    it('should handle authentication errors', async () => {
      // signInのモックでエラーを返す
      (invoke as any).mockRejectedValueOnce(new Error('Authentication failed'));

      // ストアの状態を確認
      const { getState } = useAuthStore;
      const store = getState();
      
      // サインイン（エラーが内部で処理される）
      await store.signIn('invalid-id');

      // 更新された状態を取得
      const updatedStore = getState();
      
      // ストアの状態を確認
      expect(updatedStore.isAuthenticated).toBe(false);
      expect(updatedStore.user).toBe(null);
      expect(updatedStore.error).not.toBe(null);
    });

    it('should logout correctly', () => {
      // まずログイン状態にする
      const { setState, getState } = useAuthStore;
      setState({
        user: testUser,
        isAuthenticated: true,
        isLoading: false,
        error: null
      });
      
      // 状態が正しく設定されたか確認
      let store = getState();
      expect(store.isAuthenticated).toBe(true);
      expect(store.user).toEqual(testUser);

      // ログアウト
      store.logout();

      // 更新された状態を取得
      store = getState();
      
      // ストアの状態を確認
      expect(store.isAuthenticated).toBe(false);
      expect(store.user).toBe(null);
    });
  
    describe('Profile Retrieval', () => {
      it('should retrieve user profile', async () => {
        // getProfileのモック
        (invoke as any).mockResolvedValueOnce(testUser);
  
        // auth-service.tsからgetProfile関数をインポート
        const { getProfile } = await import('@/services/auth-service');
  
        // プロフィール取得
        const profile = await getProfile(testUser.id);
  
        expect(profile).toEqual(testUser);
        expect(invoke).toHaveBeenCalledWith('get_profile', {
          userId: testUser.id,
        });
      });
  
      it('should handle non-existent profiles', async () => {
        // getProfileのモックでnullを返す
        (invoke as any).mockResolvedValueOnce(null);
  
        // auth-service.tsからgetProfile関数をインポート
        const { getProfile } = await import('@/services/auth-service');
  
        // 存在しないプロフィール
        const profile = await getProfile('non-existent-id');
  
        expect(profile).toBe(null);
      });
    });
  });
});