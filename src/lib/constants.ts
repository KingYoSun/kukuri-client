/**
 * アプリケーション定数
 */

// アプリケーション情報
export const APP_NAME = "Kukuri";
export const APP_VERSION = "0.1.0";

// ページネーション
export const DEFAULT_PAGE_SIZE = 20;
export const MAX_PAGE_SIZE = 100;

// 投稿
export const MAX_POST_LENGTH = 500;
export const MAX_SEARCH_QUERY_LENGTH = 100;

// ユーザープロフィール
export const MAX_DISPLAY_NAME_LENGTH = 50;
export const MAX_BIO_LENGTH = 160;

// ネットワーク
export const DEFAULT_RELAYS = [
  "wss://relay.example.com",
  "wss://backup-relay.example.com",
];

// ローカルストレージキー
export const STORAGE_KEYS = {
  THEME: "kukuri-theme",
  LANGUAGE: "kukuri-language",
  CURRENT_USER: "kukuri-current-user",
  SETTINGS: "kukuri-settings",
};

// テーマ
export const THEMES = {
  LIGHT: "light",
  DARK: "dark",
  SYSTEM: "system",
} as const;

// 言語
export const LANGUAGES = {
  JA: "ja",
  EN: "en",
} as const;

// ルート
export const ROUTES = {
  HOME: "/",
  AUTH: "/auth",
  PROFILE: "/profile",
  SETTINGS: "/settings",
  POST: "/post",
  SEARCH: "/search",
};

// エラーメッセージ
export const ERROR_MESSAGES = {
  NETWORK_ERROR: "ネットワークエラーが発生しました。後でもう一度お試しください。",
  AUTH_ERROR: "認証エラーが発生しました。再度ログインしてください。",
  UNKNOWN_ERROR: "不明なエラーが発生しました。後でもう一度お試しください。",
  VALIDATION_ERROR: "入力データが無効です。入力内容を確認してください。",
};

// 成功メッセージ
export const SUCCESS_MESSAGES = {
  POST_CREATED: "投稿が作成されました",
  PROFILE_UPDATED: "プロフィールが更新されました",
  SETTINGS_UPDATED: "設定が更新されました",
  USER_FOLLOWED: "ユーザーをフォローしました",
  USER_UNFOLLOWED: "ユーザーのフォローを解除しました",
};