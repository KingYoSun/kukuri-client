import { test, expect } from '@playwright/test';

test.describe('投稿機能', () => {
  // 各テスト前にログインする
  test.beforeEach(async ({ page }) => {
    // アプリケーションのホームページにアクセス
    await page.goto('/');
    
    // ページが読み込まれるまで待機
    await page.waitForLoadState('networkidle');
    
    // 既存のユーザーが存在する場合は選択してログイン
    try {
      // 最初のユーザーを選択
      await page.getByRole('button').first().click({ timeout: 60000 });
      
      // 「Sign In」ボタンをクリック
      await page.getByRole('button', { name: 'Sign In' }).click();
      
      // ホームページに遷移することを確認
      await expect(page.getByText('Home Timeline')).toBeVisible({ timeout: 60000 });
    } catch (error) {
      // ユーザーが存在しない場合は新規作成
      await page.getByText('Create a new account').click({ timeout: 60000 });
      
      // ユーザー名を入力
      const testUserName = `Test User ${Date.now()}`;
      await page.getByPlaceholder('Display Name').fill(testUserName);
      
      // 「Create Account」ボタンをクリック
      await page.getByRole('button', { name: 'Create Account' }).click();
      
      // ホームページに遷移することを確認
      await expect(page.getByText('Home Timeline')).toBeVisible({ timeout: 60000 });
    }
  });
  
  test('新規投稿を作成して表示できる', async ({ page }) => {
    // 投稿内容を入力
    const postContent = `Test post ${Date.now()} #test`;
    await page.getByPlaceholder("What's on your mind?").fill(postContent);
    
    // 「Post」ボタンをクリック
    await page.getByRole('button', { name: 'Post' }).click();
    
    // 投稿が表示されることを確認
    await expect(page.getByText(postContent)).toBeVisible({ timeout: 60000 });
    
    // ハッシュタグが表示されることを確認
    await expect(page.getByText('#test')).toBeVisible({ timeout: 60000 });
  });
  
  test('投稿一覧が表示される', async ({ page }) => {
    // ホームページに投稿一覧が表示されることを確認
    // 投稿がない場合は「No posts yet」と表示される
    try {
      await expect(page.locator('.bg-card').first()).toBeVisible({ timeout: 60000 });
    } catch (error) {
      await expect(page.getByText('No posts yet')).toBeVisible({ timeout: 60000 });
      
      // 投稿がない場合は新規投稿を作成
      const postContent = `Test post ${Date.now()}`;
      await page.getByPlaceholder("What's on your mind?").fill(postContent);
      await page.getByRole('button', { name: 'Post' }).click();
      await expect(page.getByText(postContent)).toBeVisible({ timeout: 60000 });
    }
  });
});