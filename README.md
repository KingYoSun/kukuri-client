# Kukuri Client - Tauri + React + TypeScript

このプロジェクトはTauri、React、TypeScriptを使用したデスクトップアプリケーションです。

## 開発環境のセットアップ

### Dev Containerを使用する方法（推奨）

このプロジェクトはDev Containerをサポートしています。Dev Containerを使用すると、Dockerコンテナ内で一貫した開発環境を利用できます。

#### 前提条件

- [VS Code](https://code.visualstudio.com/)
- [Docker](https://www.docker.com/products/docker-desktop/)
- [VS Code Dev Containers拡張機能](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

#### 手順

1. VS Codeでプロジェクトを開きます
2. コマンドパレット（`F1`キー）を開き、`Dev Containers: Reopen in Container`を選択します
3. VS Codeがコンテナをビルドして開発環境を準備するのを待ちます
4. 準備完了後、コンテナ内でプロジェクトが開かれます

### ローカル環境でのセットアップ

Dev Containerを使用しない場合は、以下の手順でローカル環境をセットアップできます。

#### 前提条件

- [Node.js](https://nodejs.org/) (LTS版推奨)
- [Rust](https://www.rust-lang.org/tools/install)
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

#### Tauriの依存関係をインストール

システムに応じて必要な依存関係をインストールしてください：

- [Linux](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux)
- [macOS](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-macos)
- [Windows](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-windows)

#### プロジェクトのセットアップ

```bash
# 依存関係をインストール
pnpm install

# 開発サーバーを起動
pnpm tauri dev
```

## テスト

このプロジェクトでは、Vitestを使用したユニットテスト、統合テスト、およびPlaywrightを使用したE2Eテストをサポートしています。

### テストの実行

```bash
# すべてのユニットテストと統合テストを実行
pnpm test

# テストをウォッチモードで実行（ファイル変更時に自動的に再実行）
pnpm test:watch

# テストをUIモードで実行（ブラウザベースのインターフェイスで結果を表示）
pnpm test:ui

# テストカバレッジレポートを生成
pnpm test:coverage

# E2Eテストを実行
pnpm test:e2e
```

### テストの構造

テストは以下のディレクトリ構造に従って整理されています：

```
tests/                # フロントエンドのテスト
├── unit/             # ユニットテスト
│   ├── components/   # UIコンポーネントのテスト
│   ├── hooks/        # カスタムフックのテスト
│   ├── models/       # データモデルのテスト
│   ├── services/     # サービスのテスト
│   └── stores/       # ストアのテスト
├── integration/      # 統合テスト
└── e2e/              # エンドツーエンドテスト

src-tauri/tests/      # バックエンド（Rust）のテスト
```

### Rustのテスト実行

Rustのテストは以下のコマンドで実行できます：

```bash
# すべてのRustテストを実行
cd src-tauri && cargo test

# 特定のテストを実行
cd src-tauri && cargo test test_name

# テスト実行時の出力を表示
cd src-tauri && cargo test -- --nocapture

# テスト用の機能フラグを有効にしてテストを実行
cd src-tauri && cargo test --features test-utils
```

### テストの作成

#### ユニットテスト

新しいユニットテストを作成するには、対応するディレクトリに`*.test.ts`または`*.test.tsx`ファイルを作成します。

```typescript
import { describe, it, expect } from 'vitest';

describe('機能名', () => {
  it('期待される動作', () => {
    // テストコード
    expect(true).toBe(true);
  });
});
```

#### E2Eテスト

E2Eテストは`tests/e2e`ディレクトリに作成します。

```typescript
import { test, expect } from '@playwright/test';

test('アプリケーションが正常に起動する', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('Welcome')).toBeVisible();
});
```

### モックの使用

テスト内でTauri APIをモックするには、以下のようにします：

```typescript
import { vi } from 'vitest';

// Tauriのinvokeをモック化
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

// テスト内でモックの実装を設定
(invoke as any).mockResolvedValue({ success: true });
```
