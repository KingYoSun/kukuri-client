#!/bin/bash

# MCP設定ディレクトリのパス
MCP_SETTINGS_DIR="/home/vscode/.vscode-server/data/User/globalStorage/rooveterinaryinc.roo-cline/settings"
REPO_SETTINGS_DIR="/workspaces/kukuri-client/.devcontainer/mcp-settings"

# 設定ディレクトリが存在するか確認し、存在しない場合は作成
if [ ! -d "$MCP_SETTINGS_DIR" ]; then
  echo "MCP設定ディレクトリが見つかりません。作成します: $MCP_SETTINGS_DIR"
  mkdir -p "$MCP_SETTINGS_DIR"
fi

# リポジトリの設定を現在の環境にコピー
echo "リポジトリのMCP設定を現在の環境に適用しています..."
cp -f "$REPO_SETTINGS_DIR/cline_mcp_settings.json" "$MCP_SETTINGS_DIR/"

echo "適用完了！"
echo "Roo Codeを再起動すると、新しい設定が反映されます。"