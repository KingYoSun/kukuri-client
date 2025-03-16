import { invoke } from "@tauri-apps/api/core";
import { Settings, UpdateSettingsInput } from "@/models/settings";
import { validateUpdateSettings } from "@/lib/validators";

/**
 * 設定サービス
 * 
 * アプリケーション設定の取得と更新に関連する機能を提供します。
 */
export interface SettingsUpdateResult {
  success: boolean;
  message?: string;
}

/**
 * 設定を取得します
 */
export async function getSettings(userId?: string): Promise<Settings> {
  try {
    const settings = await invoke<Settings>("get_settings", {
      userId,
    });

    return settings;
  } catch (error) {
    console.error("Error fetching settings:", error);
    throw error;
  }
}

/**
 * 設定を更新します
 */
export async function updateSettings(
  userId: string | undefined,
  input: UpdateSettingsInput
): Promise<SettingsUpdateResult> {
  const validation = validateUpdateSettings(input);
  if (!validation.success) {
    throw new Error(`Validation error: ${JSON.stringify(validation.error)}`);
  }

  try {
    const result = await invoke<SettingsUpdateResult>("update_settings", {
      userId,
      selectedRelays: input.selectedRelays,
      theme: input.theme,
      language: input.language,
      autostart: input.autostart,
      notifications: input.notifications,
    });

    return result;
  } catch (error) {
    console.error("Error updating settings:", error);
    throw error;
  }
}