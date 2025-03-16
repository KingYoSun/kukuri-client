import { z } from "zod";

// アプリケーション設定のZodスキーマ
export const settingsSchema = z.object({
  userId: z.string().uuid().optional(),
  selectedRelays: z.array(z.string().url()).default([]),
  theme: z.enum(["light", "dark", "system"]).default("system"),
  language: z.string().default("ja"),
  autostart: z.boolean().default(false),
  notifications: z.boolean().default(true),
});

// 型定義の抽出
export type Settings = z.infer<typeof settingsSchema>;

// 設定更新のためのスキーマ
export const updateSettingsSchema = z.object({
  selectedRelays: z.array(z.string().url()).optional(),
  theme: z.enum(["light", "dark", "system"]).optional(),
  language: z.string().optional(),
  autostart: z.boolean().optional(),
  notifications: z.boolean().optional(),
});

export type UpdateSettingsInput = z.infer<typeof updateSettingsSchema>;