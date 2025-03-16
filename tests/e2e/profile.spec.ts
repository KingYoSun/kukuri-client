import { test, expect } from '@playwright/test';

test.describe('プロファイル機能', () => {
  let userId: string;

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
    
    // URLからユーザーIDを取得（後でプロファイルページにアクセスするため）
    // 注: 実際のアプリケーションの実装によってはこの方法が機能しない場合があります
    // その場合は、別の方法でユーザーIDを取得する必要があります
    userId = await page.evaluate(() => {
      // @ts-ignore
      return window.localStorage.getItem('userId') || '';
    });
  });
  
  test('プロファイルページにアクセスして表示できる', async ({ page }) => {
    // プロファイルページにアクセス
    await page.goto(`/profile/${userId}`);
    
    // ページが読み込まれるまで待機
    await page.waitForLoadState('networkidle');
    
    // プロファイル情報が表示されることを確認
    await expect(page.getByText('Following')).toBeVisible({ timeout: 60000 });
    await expect(page.getByText('Followers')).toBeVisible({ timeout: 60000 });
    await expect(page.getByText('Posts')).toBeVisible({ timeout: 60000 });
    
    // 「Edit Profile」ボタンが表示されることを確認（自分のプロファイルの場合）
    await expect(page.getByRole('button', { name: 'Edit Profile' })).toBeVisible({ timeout: 60000 });
  });
  
  test('プロファイル情報を編集できる', async ({ page }) => {
    // プロファイルページにアクセス
    await page.goto(`/profile/${userId}`);
    
    // ページが読み込まれるまで待機
    await page.waitForLoadState('networkidle');
    
    // 「Edit Profile」ボタンをクリック
    await page.getByRole('button', { name: 'Edit Profile' }).click({ timeout: 60000 });
    
    // 新しいプロファイル情報を入力
    const newDisplayName = `Updated User ${Date.now()}`;
    const newBio = `Updated bio ${Date.now()}`;
    
    await page.getByLabel('Display Name').fill(newDisplayName);
    await page.getByLabel('Bio').fill(newBio);
    
    // 「Save」ボタンをクリック
    await page.getByRole('button', { name: 'Save' }).click();
    
    // 更新されたプロファイル情報が表示されることを確認
    await expect(page.getByText(newDisplayName)).toBeVisible({ timeout: 60000 });
    await expect(page.getByText(newBio)).toBeVisible({ timeout: 60000 });
  });
});