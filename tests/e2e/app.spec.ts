import { test, expect } from '@playwright/test';

test.describe('基本的なアプリケーション機能', () => {
  test('アプリケーションが正常に起動する', async ({ page }) => {
    // アプリケーションのホームページにアクセス
    await page.goto('/');
    
    // ページが読み込まれるまで待機
    await page.waitForLoadState('networkidle');
    
    // Kukuriのタイトルが表示されることを確認
    await expect(page.getByText('Kukuri')).toBeVisible({ timeout: 60000 });
    
    // 認証ページが表示されることを確認（初回アクセス時）
    await expect(page.getByText('Choose your connections, control your network')).toBeVisible({ timeout: 60000 });
    
    // ログインフォームが表示されることを確認
    await expect(page.getByRole('heading', { name: 'ログイン' })).toBeVisible({ timeout: 60000 });
    
    // 「新しいアカウントを作成」リンクが表示されることを確認
    await expect(page.getByRole('link', { name: '新しいアカウントを作成' })).toBeVisible({ timeout: 60000 });
  });
});