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
