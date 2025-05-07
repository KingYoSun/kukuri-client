#!/bin/bash

# MCP設定ディレクトリのパス
MCP_SETTINGS_DIR="/home/vscode/.vscode-server/data/User/globalStorage/rooveterinaryinc.roo-cline/settings"
REPO_SETTINGS_DIR="/workspaces/kukuri-client/.devcontainer/mcp-settings"

# 設定ディレクトリが存在するか確認
if [ ! -d "$MCP_SETTINGS_DIR" ]; then
  echo "エラー: MCP設定ディレクトリが見つかりません: $MCP_SETTINGS_DIR"
  exit 1
fi

# 現在のMCP設定をリポジトリにコピー
echo "現在のMCP設定をリポジトリに同期しています..."
cp -f "$MCP_SETTINGS_DIR/cline_mcp_settings.json" "$REPO_SETTINGS_DIR/"

echo "同期完了！"
echo "変更をコミットしてプッシュすることで、チーム全体でMCP設定を共有できます。"