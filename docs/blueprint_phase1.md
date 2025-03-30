## 目次
1. [プロジェクト概要](#1-プロジェクト概要)
2. [開発環境設定](#2-開発環境設定)
3. [アプリケーション構造](#3-アプリケーション構造)
4. [技術実装詳細](#4-技術実装詳細)
5. [データモデル設計](#5-データモデル設計)
6. [テスト戦略](#6-テスト戦略)
7. [CI/CD パイプライン](#7-cicd-パイプライン)
8. [実装ロードマップ](#8-実装ロードマップ)
9. [.clinerulesファイル](#9-clinerulesファイル)

## 1. プロジェクト概要

分散型ソーシャルネットワークアプリケーションのPhase 1（MVP）を実装するための詳細設計書です。この実装は、Tauri、iroh-gossip、iroh-docs、iroh-blobsなどの技術を使用して、分散型のソーシャルネットワークの基本機能を提供します。MVPでは以下の機能を実装します：

- ユーザープロフィール作成と管理
- 短文テキスト投稿とタイムライン表示
- 基本的なフォロー/フォロワー機能
- シンプルな投稿検索（ローカルのみ）
- 基本的なP2P通信（iroh-gossip使用）
- ローカルデータの保存と同期（iroh-docs使用）
- バイナリデータの管理（iroh-blobs使用）

## 2. 開発環境設定

### 2.1 必要なツール・ライブラリ

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

### 2.2 プロジェクト初期化

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

## 3. アプリケーション構造

### 3.1 ディレクトリ構造

```
decentralized-social-app/
├── .github/                      # GitHub Actions設定
│   └── workflows/                # CI/CDワークフロー定義
├── src/                          # Reactアプリケーションのソースコード
│   ├── assets/                   # 静的アセット
│   ├── components/               # UIコンポーネント
│   │   ├── ui/                   # shadcn/uiコンポーネント
│   │   ├── auth/                 # 認証関連コンポーネント
│   │   ├── post/                 # 投稿関連コンポーネント
│   │   ├── profile/              # プロフィール関連コンポーネント
│   │   └── layout/               # レイアウトコンポーネント
│   ├── hooks/                    # カスタムReactフック
│   │   ├── use-auth.ts           # 認証フック
│   │   ├── use-posts.ts          # 投稿関連フック
│   │   └── use-profile.ts        # プロフィール関連フック
│   ├── lib/                      # ユーティリティ関数とヘルパー
│   │   ├── utils.ts              # 一般ユーティリティ
│   │   ├── validators.ts         # zodバリデータ
│   │   └── constants.ts          # 定数
│   ├── models/                   # データモデル（zod型定義）
│   │   ├── user.ts               # ユーザーモデル
│   │   ├── post.ts               # 投稿モデル
│   │   └── settings.ts           # 設定モデル
│   ├── pages/                    # ページコンポーネント
│   │   ├── home/                 # ホームページ
│   │   ├── profile/              # プロフィールページ
│   │   ├── settings/             # 設定ページ
│   │   └── auth/                 # 認証ページ
│   ├── services/                 # サービス層
│   │   ├── auth-service.ts       # 認証サービス
│   │   ├── post-service.ts       # 投稿サービス
│   │   ├── profile-service.ts    # プロフィールサービス
│   │   └── storage-service.ts    # ストレージサービス
│   ├── stores/                   # Zustandストア
│   │   ├── auth-store.ts         # 認証状態
│   │   ├── post-store.ts         # 投稿状態
│   │   └── settings-store.ts     # 設定状態
│   ├── App.tsx                   # メインアプリケーションコンポーネント
│   ├── main.tsx                  # エントリーポイント
│   └── vite-env.d.ts             # Viteの型定義
├── src-tauri/                    # Tauriネイティブ部分
│   ├── src/                      # Rustコード
│   │   ├── main.rs               # メインエントリーポイント
│   │   ├── commands/             # Tauriコマンド
│   │   │   ├── mod.rs            # コマンドモジュール
│   │   │   ├── auth.rs           # 認証コマンド
│   │   │   ├── post.rs           # 投稿コマンド
│   │   │   └── settings.rs       # 設定コマンド
│   │   ├── storage/              # ストレージ関連
│   │   │   ├── mod.rs            # ストレージモジュール
│   │   │   └── iroh_docs_sync.rs # iroh-docs実装
│   │   ├── network/              # ネットワーク関連
│   │   │   ├── mod.rs            # ネットワークモジュール
│   │   │   └── iroh.rs           # iroh-gossip実装
│   │   └── models/               # Rustデータモデル
│   │       ├── mod.rs            # モデルモジュール
│   │       ├── user.rs           # ユーザーモデル
│   │       └── post.rs           # 投稿モデル
│   ├── Cargo.toml                # Rust依存関係
│   └── tauri.conf.json           # Tauri設定
├── tests/                        # テストディレクトリ
│   ├── unit/                     # ユニットテスト
│   ├── integration/              # 統合テスト
│   └── e2e/                      # E2Eテスト
├── .clinerulesrules              # AIへの行動規範設定
├── .gitignore                    # Gitの無視ファイル
├── package.json                  # NPM依存関係
├── tailwind.config.js            # Tailwind設定
├── tsconfig.json                 # TypeScript設定
├── vite.config.ts                # Vite設定
└── README.md                     # プロジェクト説明
```

### 3.2 主要コンポーネントの関係

```mermaid
graph TD
    A[App.tsx] --> B[Router]
    B --> C[Pages]
    C --> D[Components]
    D --> E[Hooks]
    E --> F[Services]
    F --> G[Tauri Commands]
    G --> H[Rust Backend]
    H --> I[iroh-docs Storage]
    H --> J[iroh-gossip Network]
    H --> K[iroh-blobs Content]
    E --> L[Zustand Stores]