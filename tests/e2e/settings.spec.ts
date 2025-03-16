import { test, expect } from '@playwright/test';

test.describe('設定機能', () => {
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
    
    // 設定ページにアクセス
    await page.goto('/settings');
    
    // ページが読み込まれるまで待機
    await page.waitForLoadState('networkidle');
    
    // 設定ページが表示されることを確認
    await expect(page.getByText('Settings')).toBeVisible({ timeout: 60000 });
  });
  
  test('テーマを変更できる', async ({ page }) => {
    // 「Appearance」セクションが表示されることを確認
    await expect(page.getByText('Appearance')).toBeVisible({ timeout: 60000 });
    
    // 「Dark」テーマボタンをクリック
    await page.getByRole('button', { name: 'Dark' }).click();
    
    // ダークテーマが適用されることを確認
    // 注: 実際のテーマ変更の確認方法はアプリケーションの実装によって異なります
    // ここでは、ボタンの状態で確認しています
    await expect(page.getByRole('button', { name: 'Dark' })).toHaveClass(/default/, { timeout: 60000 });
    
    // 「Light」テーマボタンをクリック
    await page.getByRole('button', { name: 'Light' }).click();
    
    // ライトテーマが適用されることを確認
    await expect(page.getByRole('button', { name: 'Light' })).toHaveClass(/default/, { timeout: 60000 });
    
    // 「System」テーマボタンをクリック
    await page.getByRole('button', { name: 'System' }).click();
    
    // システムテーマが適用されることを確認
    await expect(page.getByRole('button', { name: 'System' })).toHaveClass(/default/, { timeout: 60000 });
  });
  
  test('リレーを追加・削除できる', async ({ page }) => {
    // 「Network」セクションが表示されることを確認
    await expect(page.getByText('Network')).toBeVisible({ timeout: 60000 });
    
    // 「Add Relay」ボタンをクリック
    await page.getByRole('button', { name: 'Add Relay' }).click();
    
    // リレーURLを入力
    const testRelayUrl = `https://test-relay-${Date.now()}.example.com`;
    await page.getByPlaceholder('https://relay.example.com').fill(testRelayUrl);
    
    // 「Add」ボタンをクリック
    await page.getByRole('button', { name: 'Add' }).click();
    
    // 追加したリレーが表示されることを確認
    await expect(page.getByText(testRelayUrl)).toBeVisible({ timeout: 60000 });
    
    // 「Remove」ボタンをクリック
    await page.getByText(testRelayUrl).locator('..').getByRole('button', { name: 'Remove' }).click();
    
    // リレーが削除されることを確認
    await expect(page.getByText(testRelayUrl)).not.toBeVisible({ timeout: 60000 });
  });
  
  test('ログアウトできる', async ({ page }) => {
    // 「Account」セクションが表示されることを確認
    await expect(page.getByText('Account')).toBeVisible({ timeout: 60000 });
    
    // 「Logout」ボタンをクリック
    await page.getByRole('button', { name: 'Logout' }).click();
    
    // 認証ページに戻ることを確認
    await expect(page.getByText('Kukuri')).toBeVisible({ timeout: 60000 });
    await expect(page.getByText('Choose your connections, control your network')).toBeVisible({ timeout: 60000 });
  });
});