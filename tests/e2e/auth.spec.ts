import { test, expect } from '@playwright/test';

test.describe('認証機能', () => {
  test('アカウント作成モードに切り替えることができる', async ({ page }) => {
    // アプリケーションのホームページにアクセス
    await page.goto('/');
    
    // ページが読み込まれるまで待機
    await page.waitForLoadState('networkidle');
    
    // 「新しいアカウントを作成」リンクをクリック
    await page.getByRole('link', { name: '新しいアカウントを作成' }).click({ timeout: 60000 });
    
    // アカウント作成モードに切り替わったことを確認
    await expect(page.getByRole('heading', { name: 'アカウント作成' })).toBeVisible({ timeout: 60000 });
    
    // ユーザーIDフィールドが表示されることを確認
    await expect(page.getByLabel('ユーザーID')).toBeVisible({ timeout: 60000 });
    
    // 「アカウント作成」ボタンが表示されることを確認
    await expect(page.getByRole('button', { name: 'アカウント作成' })).toBeVisible({ timeout: 60000 });
    
    // 「既存のアカウントでログイン」リンクが表示されることを確認
    await expect(page.getByRole('link', { name: '既存のアカウントでログイン' })).toBeVisible({ timeout: 60000 });
  });
});