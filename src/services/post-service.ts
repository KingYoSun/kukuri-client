import { invoke } from "@tauri-apps/api/core";
import { CreatePostInput, Post, SearchPostsInput } from "@/models/post";
import { validateCreatePost, validateSearchPosts } from "@/lib/validators";
import { DEFAULT_PAGE_SIZE } from "@/lib/constants";

/**
 * 投稿サービス
 * 
 * 投稿の作成、取得、検索に関連する機能を提供します。
 */
export interface PostResult {
  postId: string;
  success: boolean;
  message?: string;
}

/**
 * 新しい投稿を作成します
 */
export async function createPost(authorId: string, input: CreatePostInput): Promise<PostResult> {
  const validation = validateCreatePost(input);
  if (!validation.success) {
    throw new Error(`Validation error: ${JSON.stringify(validation.error)}`);
  }

  try {
    const result = await invoke<PostResult>("create_post", {
      authorId,
      content: input.content,
      attachments: input.attachments || [],
    });

    return result;
  } catch (error) {
    console.error("Error creating post:", error);
    throw error;
  }
}

/**
 * すべての投稿を取得します
 */
export async function getPosts(limit = DEFAULT_PAGE_SIZE, offset = 0): Promise<Post[]> {
  try {
    const posts = await invoke<Post[]>("get_posts", {
      limit,
      offset,
    });

    return posts;
  } catch (error) {
    console.error("Error fetching posts:", error);
    throw error;
  }
}

/**
 * 特定のユーザーの投稿を取得します
 */
export async function getUserPosts(userId: string, limit = DEFAULT_PAGE_SIZE, offset = 0): Promise<Post[]> {
  try {
    const posts = await invoke<Post[]>("get_user_posts", {
      userId,
      limit,
      offset,
    });

    return posts;
  } catch (error) {
    console.error("Error fetching user posts:", error);
    throw error;
  }
}

/**
 * 投稿を検索します
 */
export async function searchPosts(input: SearchPostsInput): Promise<Post[]> {
  const validation = validateSearchPosts(input);
  if (!validation.success) {
    throw new Error(`Validation error: ${JSON.stringify(validation.error)}`);
  }

  try {
    const posts = await invoke<Post[]>("search_posts", {
      query: input.query,
      limit: input.limit,
    });

    return posts;
  } catch (error) {
    console.error("Error searching posts:", error);
    throw error;
  }
}