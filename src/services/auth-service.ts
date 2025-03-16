import { invoke } from "@tauri-apps/api/core";
import { CreateUserInput, User } from "@/models/user";
import { validateCreateUser, validateUser } from "@/lib/validators";

/**
 * 認証サービス
 * 
 * ユーザー認証に関連する機能を提供します。
 */
export interface AuthResult {
  userId: string;
  success: boolean;
  message?: string;
}

export interface UserListItem {
  id: string;
  displayName: string;
}

/**
 * 新しいユーザーを作成します
 */
export async function createUser(input: CreateUserInput): Promise<AuthResult> {
  const validation = validateCreateUser(input);
  if (!validation.success) {
    throw new Error(`Validation error: ${JSON.stringify(validation.error)}`);
  }

  try {
    const result = await invoke<AuthResult>("create_user", {
      displayName: input.displayName,
      bio: input.bio,
    });

    return result;
  } catch (error) {
    console.error("Error creating user:", error);
    throw error;
  }
}

/**
 * 既存のユーザーでサインインします
 */
export async function signIn(userId: string): Promise<AuthResult> {
  try {
    const result = await invoke<AuthResult>("sign_in", {
      userId,
    });

    return result;
  } catch (error) {
    console.error("Error signing in:", error);
    throw error;
  }
}

/**
 * ユーザープロフィールを取得します
 */
export async function getProfile(userId: string): Promise<User | null> {
  try {
    const user = await invoke<User | null>("get_profile", {
      userId,
    });

    if (user) {
      const validation = validateUser(user);
      if (!validation.success) {
        console.error("Invalid user data:", validation.error);
        return null;
      }
      return user;
    }

    return null;
  } catch (error) {
    console.error("Error fetching profile:", error);
    throw error;
  }
}

/**
 * 利用可能なすべてのユーザーのリストを取得します
 */
export async function listUsers(): Promise<UserListItem[]> {
  try {
    const users = await invoke<UserListItem[]>("list_users");
    return users;
  } catch (error) {
    console.error("Error listing users:", error);
    throw error;
  }
}