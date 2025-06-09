# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリのコードを操作する際のガイダンスを提供します。

## 基本ルール

- **言語**: 必ず日本語で回答する
- **Cargoコマンド**: cargoコマンド実行時のタイムアウトは6000000ms（10分）に設定する

## プロジェクト概要

Kukuri Clientは以下の技術で構築された分散型ソーシャルネットワークデスクトップアプリケーションです：
- **フロントエンド**: React + TypeScript + Vite + Tailwind CSS
- **バックエンド**: Tauri (Rust) + iroh P2Pプロトコル
- **アーキテクチャ**: ローカルファースト、分散型、中央サーバーなし

## 必須コマンド

### 開発
```bash
# 依存関係のインストール
pnpm install

# 開発開始（Tauriウィンドウを開く）
pnpm tauri dev

# フロントエンドのみの開発
pnpm dev

# プロダクション用ビルド
pnpm tauri build
```

### テスト
```bash
# 全フロントエンドテストを実行
pnpm test

# ウォッチモードでテスト実行
pnpm test:watch

# E2Eテスト実行
pnpm test:e2e

# Rustテスト実行
cd src-tauri && cargo test

# テストユーティリティありでRustテスト実行
cd src-tauri && cargo test --features test-utils
```

### リント & 型チェック
```bash
# TypeScript型チェック
pnpm tsc --noEmit

# Rustリント
cd src-tauri && cargo clippy
```

## アーキテクチャ概要

### データフローパターン
1. **ユーザーアクション** → Reactコンポーネント → カスタムフック → サービス層
2. **サービス層** → Tauriコマンド（`invoke`経由）
3. **Rustコマンドハンドラー** → リポジトリ → irohストレージ
4. **レスポンス** → 各層を通ってUIに戻る

### 主要なアーキテクチャ決定
- **状態管理**: Zustandストア（`src/stores/`）
- **データ検証**: モデル内のZodスキーマ
- **P2Pストレージ**: 同期データ用iroh-docs、ファイル用iroh-blobs
- **イベントシステム**: Tauriのイベントシステム経由のドキュメントイベント

### ストレージアーキテクチャ
- **ユーザー**: `users`名前空間に保存
- **投稿**: `posts`名前空間に保存  
- **設定**: `settings`名前空間に保存
- **バイナリデータ**: iroh-blobsに保存

### テスト戦略
- **ユニットテスト**: `tests/unit/`内のコンポーネント、フック、モデル
- **統合テスト**: `tests/integration/`内の完全なワークフロー
- **E2Eテスト**: `tests/e2e/`内のユーザージャーニー

## 一般的な開発タスク

### 新機能の追加
1. `src/models/`でTypeScriptモデルを定義
2. `src-tauri/src/models/`でRustモデルを作成
3. `src-tauri/src/commands/`でTauriコマンドを追加
4. `src-tauri/src/storage/repository/`でリポジトリを作成
5. `src/services/`でサービスを追加
6. `src/stores/`でストアを作成
7. UIコンポーネントとフックを構築

### irohでの作業
- ドキュメントはストレージ層の`IrohNode`経由でアクセス
- 各データタイプは独自の名前空間を持つ
- 同期はirohプロトコル経由で自動実行
- バイナリデータはコンテンツアドレス指定ストレージ用iroh-blobsを使用

### 単一テストの実行
```bash
# フロントエンド: 特定のテストファイルを実行
pnpm test src/services/storage-service.test.ts

# Rust: 特定のテストを実行
cd src-tauri && cargo test test_name
```

## 重要なパターン

### Tauriコマンド
コマンドは以下のパターンに従います：
```rust
#[tauri::command]
pub async fn command_name(
    state: State<'_, AppState>,
    param: Type
) -> Result<ReturnType, String>
```

### リポジトリパターン
各データタイプには、iroh-docsと相互作用する標準的なCRUD操作を持つリポジトリモジュールがあります。

### イベント購読
ドキュメントの変更は、フロントエンドが`useDocumentEvents`フック経由で購読するイベントを発行します。

## メモリバンク使用法

`memory-bank/`ディレクトリには重要なプロジェクトコンテキストとドキュメントが含まれています：

### 利用可能なコンテキストファイル
- **projectbrief.md**: MVP要件と実装フェーズ
- **systemPatterns.md**: アーキテクチャパターンとデータフロー図
- **techContext.md**: 技術的決定と実装詳細
- **activeContext.md**: 現在の開発状況とアクティブなタスク
- **progress.md**: 開発進捗追跡
- **productContext.md**: プロダクトビジョンとユーザーエクスペリエンス目標

### メモリバンクを参照するタイミング
1. **新機能実装前**: MVP目標との整合性についてprojectbrief.mdを確認
2. **アーキテクチャ決定**: 確立されたパターンについてsystemPatterns.mdを参照
3. **データフロー実装**: systemPatterns.mdで文書化されたパターンに従う
4. **技術的選択**: 技術選択の根拠についてtechContext.mdを参照

### メモリバンクガイドライン
- systemPatterns.mdで確立されたパターンに常に従う
- 新機能がprojectbrief.mdのMVPスコープと整合することを確認
- 重要なマイルストーン完了時にprogress.mdを更新
- 現在の開発優先事項についてactiveContext.mdを参照