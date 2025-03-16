import { z } from "zod";

// ユーザープロフィールのZodスキーマ
export const userSchema = z.object({
  id: z.string().uuid(),
  displayName: z.string().min(1).max(50),
  bio: z.string().max(160).default(""),
  avatar: z.string().nullable().optional(),
  following: z.array(z.string()).default([]),
  followers: z.array(z.string()).default([]),
  createdAt: z.number(),
  updatedAt: z.number().optional(),
});

// 型定義の抽出
export type User = z.infer<typeof userSchema>;

// ユーザープロフィール作成のためのスキーマ
export const createUserSchema = z.object({
  displayName: z.string().min(1).max(50),
  bio: z.string().max(160).optional(),
});

export type CreateUserInput = z.infer<typeof createUserSchema>;

// プロフィール更新のためのスキーマ
export const updateProfileSchema = z.object({
  displayName: z.string().min(1).max(50).optional(),
  bio: z.string().max(160).optional(),
  avatar: z.string().optional(),
});

export type UpdateProfileInput = z.infer<typeof updateProfileSchema>;