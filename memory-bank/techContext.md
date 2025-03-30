# 技術コンテキスト

## 技術スタック概要

本プロジェクトは以下の主要な技術スタックを使用しています：

### フロントエンド
- **フレームワーク**: React 18.3.1
- **言語**: TypeScript 5.6.2
- **ビルドツール**: Vite 6.0.3
- **状態管理**: Zustand 5.0.3
- **データフェッチング**: TanStack Query 5.68.0
- **ルーティング**: React Router DOM 7.3.0
- **UIコンポーネント**: Shadcn/UI (Radix UI)
- **スタイリング**: TailwindCSS 4.0.14
- **バリデーション**: Zod 3.24.2

### バックエンド
- **アプリケーションフレームワーク**: Tauri 2.x
- **言語**: Rust
- **分散通信プロトコル**: iroh-gossip
- **データストレージ**: iroh-docs
- **コンテンツストレージ**: iroh-blobs
- **リレーインフラストラクチャ**: Cloudflare Workers

### テスト
- **ユニットテスト**: Vitest 1.4.0
- **E2Eテスト**: Playwright 1.42.1
- **テストユーティリティ**: Testing Library 16.2.0

## 開発環境セットアップ

### 必要なツール・ライブラリ

```bash
# Rustツールチェーン
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# Node.js (v18以上推奨)
# nvm経由での設定例
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
nvm install 18
nvm use 18

# Tauriの依存関係（Ubuntu/Debian系）
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# macOS依存関係
xcode-select --install

# Windows依存関係
# WebViewインストーラーをダウンロードして実行
# https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

### プロジェクト初期化

```bash
# ディレクトリ作成
mkdir decentralized-social-app
cd decentralized-social-app

# Tauriプロジェクト初期化
npm create tauri-app@latest .
# Vite + Reactを選択
# TypeScriptを選択

# 必要な依存関係のインストール
npm install zustand @tanstack/react-query react-router-dom zod
npm install @tauri-apps/api iroh iroh-gossip iroh-docs iroh-blobs

# 開発ツールのインストール
npm install -D tailwindcss postcss autoprefixer shadcn-ui vitest
npx tailwindcss init -p
npx shadcn-ui init
```

## 主要技術の詳細

### Tauri

Tauriは、Webテクノロジーを使用してデスクトップアプリケーションを構築するためのフレームワークです。Electronと比較して、より軽量で高パフォーマンスなアプリケーションを作成できます。

- **バージョン**: 2.x
- **主な特徴**:
  - クロスプラットフォーム対応（Windows, macOS, Linux）
  - 軽量で高パフォーマンス
  - セキュリティ重視の設計
  - Rustバックエンドとの統合

### iroh-docs

iroh-docsは、マルチディメンショナルなキーバリュードキュメントを提供するライブラリで、効率的な同期プロトコルを持っています。

- **主な特徴**:
  - 効率的な同期プロトコル
  - レプリカを使用したデータ同期
  - 認証と暗号化
  - オフライン対応
  - iroh-gossipとiroh-blobsとの統合

### iroh-gossip

iroh-gossipは、分散型ネットワークのためのP2P通信プロトコルです。トピックベースのメッセージングを提供し、効率的なデータ配信を実現します。

- **主な特徴**:
  - トピックベースのメッセージング
  - 効率的なP2P通信
  - スケーラブルな設計
  - 低レイテンシー

### iroh-blobs

iroh-blobsは、コンテンツアドレス可能なバイナリデータストレージを提供するライブラリです。

- **主な特徴**:
  - コンテンツアドレス可能なストレージ
  - 効率的なデータ転送
  - 重複排除
  - iroh-docsとの統合

### React + TypeScript

ReactとTypeScriptの組み合わせにより、型安全で保守性の高いフロントエンドコードを実現します。

- **Reactバージョン**: 18.3.1
- **TypeScriptバージョン**: 5.6.2
- **主な特徴**:
  - コンポーネントベースのUI開発
  - 型安全なコード
  - 効率的なレンダリング
  - 豊富なエコシステム

### Zustand

Zustandは、軽量で使いやすい状態管理ライブラリです。Reduxと比較してボイラープレートが少なく、シンプルなAPIを提供します。

- **バージョン**: 5.0.3
- **主な特徴**:
  - 軽量で高パフォーマンス
  - シンプルなAPI
  - Reactとの統合が容易
  - デバッグが容易

## 技術的制約と考慮事項

### パフォーマンス

- **メモリ使用量**: iroh-docsのデータ構造は、大規模なデータセットでメモリ使用量が増加する可能性があります。データの分割と効率的な管理が必要です。
- **CPU使用量**: P2P通信は、CPU使用量が増加する可能性があります。効率的なアルゴリズムと最適化が必要です。
- **ネットワーク帯域**: P2P通信は、ネットワーク帯域を消費します。効率的なデータ同期メカニズムが必要です。

### セキュリティ

- **データ保護**: ローカルに保存されるデータの保護が必要です。
- **通信セキュリティ**: P2P通信の暗号化と認証が必要です。
- **権限管理**: 適切なアクセス制御と権限管理が必要です。

### 互換性

- **クロスプラットフォーム**: Windows、macOS、Linuxでの動作確認が必要です。
- **バージョン互換性**: 異なるバージョンのクライアント間での互換性を確保する必要があります。

## 開発ツールとワークフロー

### 開発サーバー

```bash
# 開発サーバーの起動
npm run tauri dev
```

### ビルド

```bash
# プロダクションビルド
npm run tauri build
```

### テスト

```bash
# ユニットテスト
npm run test

# E2Eテスト
npm run test:e2e
```

### CI/CD

GitHub Actionsを使用して、以下のワークフローを自動化します：

1. **プルリクエストチェック**: コードの品質チェックとテスト
2. **メインブランチ統合**: 統合テストとビルド
3. **リリースビルド**: クロスプラットフォームビルドとリリース

## 依存関係管理

- **フロントエンド**: npm/pnpmを使用
- **バックエンド**: Cargoを使用

## デプロイメント

- **デスクトップアプリケーション**: Tauriのビルド機能を使用して、各プラットフォーム向けのインストーラーを作成
- **自動更新**: Tauriの自動更新機能を使用して、アプリケーションの更新を提供

## モニタリングと分析

- **エラー追跡**: アプリケーション内でのエラーログ記録
- **使用状況分析**: プライバシーを尊重した匿名の使用状況データ収集（オプトイン）
- **パフォーマンスモニタリング**: リソース使用量のモニタリング