import { z } from "zod";
import { createPostSchema, postSchema, searchPostsSchema } from "@/models/post";
import { createUserSchema, updateProfileSchema, userSchema } from "@/models/user";
import { settingsSchema, updateSettingsSchema } from "@/models/settings";

/**
 * 投稿バリデーション関数
 */
export const validatePost = (data: unknown) => {
  return postSchema.safeParse(data);
};

export const validateCreatePost = (data: unknown) => {
  return createPostSchema.safeParse(data);
};

export const validateSearchPosts = (data: unknown) => {
  return searchPostsSchema.safeParse(data);
};

/**
 * ユーザーバリデーション関数
 */
export const validateUser = (data: unknown) => {
  return userSchema.safeParse(data);
};

export const validateCreateUser = (data: unknown) => {
  return createUserSchema.safeParse(data);
};

export const validateUpdateProfile = (data: unknown) => {
  return updateProfileSchema.safeParse(data);
};

/**
 * 設定バリデーション関数
 */
export const validateSettings = (data: unknown) => {
  return settingsSchema.safeParse(data);
};

export const validateUpdateSettings = (data: unknown) => {
  return updateSettingsSchema.safeParse(data);
};

/**
 * エラーメッセージのフォーマット
 */
export const formatZodError = (error: z.ZodError) => {
  return error.errors.map((err) => {
    return {
      path: err.path.join("."),
      message: err.message,
    };
  });
};