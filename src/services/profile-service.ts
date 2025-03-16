import { invoke } from "@tauri-apps/api/core";
import { UpdateProfileInput, User } from "@/models/user";
import { validateUpdateProfile } from "@/lib/validators";

/**
 * プロフィールサービス
 * 
 * ユーザープロフィールの更新、フォロー関連の機能を提供します。
 */
export interface ProfileUpdateResult {
  success: boolean;
  message?: string;
}

/**
 * プロフィールを更新します
 */
export async function updateProfile(
  userId: string,
  input: UpdateProfileInput
): Promise<ProfileUpdateResult> {
  const validation = validateUpdateProfile(input);
  if (!validation.success) {
    throw new Error(`Validation error: ${JSON.stringify(validation.error)}`);
  }

  try {
    const result = await invoke<ProfileUpdateResult>("update_profile", {
      userId,
      displayName: input.displayName,
      bio: input.bio,
      avatar: input.avatar,
    });

    return result;
  } catch (error) {
    console.error("Error updating profile:", error);
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

    return user;
  } catch (error) {
    console.error("Error fetching profile:", error);
    throw error;
  }
}

/**
 * ユーザーをフォローします
 */
export async function followUser(
  userId: string,
  targetUserId: string
): Promise<ProfileUpdateResult> {
  try {
    const result = await invoke<ProfileUpdateResult>("follow_user", {
      userId,
      targetUserId,
    });

    return result;
  } catch (error) {
    console.error("Error following user:", error);
    throw error;
  }
}

/**
 * ユーザーのフォローを解除します
 */
export async function unfollowUser(
  userId: string,
  targetUserId: string
): Promise<ProfileUpdateResult> {
  try {
    const result = await invoke<ProfileUpdateResult>("unfollow_user", {
      userId,
      targetUserId,
    });

    return result;
  } catch (error) {
    console.error("Error unfollowing user:", error);
    throw error;
  }
}