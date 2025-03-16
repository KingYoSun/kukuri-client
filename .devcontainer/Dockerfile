FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Tauriの開発に必要なシステム依存関係をインストール
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    libwebkit2gtk-4.0-dev \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.0-dev \
    libjavascriptcoregtk-4.1-dev \
    libglib2.0-dev \
    libatk1.0-dev \
    libgdk-pixbuf2.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    patchelf \
    # Puppeteer (Chromium) に必要なライブラリを追加
    libnss3 \
    libatk-bridge2.0-0 \
    libcups2 \
    libdrm2 \
    libxkbcommon0 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxrandr2 \
    libgbm1 \
    libasound2 \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# vscodeユーザー用のディレクトリを準備
RUN mkdir -p /home/vscode/.cargo \
    && chown -R vscode:vscode /home/vscode

# vscodeユーザーとしてRustツールチェーンをインストール
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup default stable \
    && rustup component add rust-src rustfmt clippy

# 環境変数を設定
ENV PATH="/home/vscode/.cargo/bin:${PATH}"
ENV CARGO_HOME="/home/vscode/.cargo"
ENV RUSTUP_HOME="/home/vscode/.rustup"

# rootユーザーに戻る（後続の操作のため）
USER root

# Tauri CLIはpostCreateCommandでインストールするため、ここでは不要

# 作業ディレクトリを設定
WORKDIR /workspaces/kukuri-client

# コンテナ起動時のデフォルトコマンド（vscodeユーザーとして実行）
USER vscode
CMD ["bash"]