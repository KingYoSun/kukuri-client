import { z } from "zod";

// 投稿のZodスキーマ
export const postSchema = z.object({
  id: z.string().uuid(),
  authorId: z.string().uuid(),
  content: z.string().min(1).max(500),
  attachments: z.array(z.string()).default([]),
  mentions: z.array(z.string()).default([]),
  hashtags: z.array(z.string()).default([]),
  createdAt: z.number(),
});

// 型定義の抽出
export type Post = z.infer<typeof postSchema>;

// 投稿作成のためのスキーマ
export const createPostSchema = z.object({
  content: z.string().min(1).max(500),
  attachments: z.array(z.string()).optional(),
});

export type CreatePostInput = z.infer<typeof createPostSchema>;

// 投稿検索のためのスキーマ
export const searchPostsSchema = z.object({
  query: z.string().min(1).max(100),
  limit: z.number().positive().max(100).optional(),
});

export type SearchPostsInput = z.infer<typeof searchPostsSchema>;