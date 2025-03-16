import { describe, it, expect } from 'vitest';
import { userSchema, createUserSchema, updateProfileSchema } from '@/models/user';

describe('User Model Validation', () => {
  describe('userSchema', () => {
    it('should validate a valid user', () => {
      const validUser = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        displayName: 'Test User',
        bio: 'This is a test bio',
        following: [],
        followers: [],
        createdAt: Date.now(),
      };

      const result = userSchema.safeParse(validUser);
      expect(result.success).toBe(true);
    });

    it('should reject a user with invalid id', () => {
      const invalidUser = {
        id: 'not-a-uuid',
        displayName: 'Test User',
        bio: 'This is a test bio',
        following: [],
        followers: [],
        createdAt: Date.now(),
      };

      const result = userSchema.safeParse(invalidUser);
      expect(result.success).toBe(false);
    });

    it('should reject a user with empty display name', () => {
      const invalidUser = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        displayName: '',
        bio: 'This is a test bio',
        following: [],
        followers: [],
        createdAt: Date.now(),
      };

      const result = userSchema.safeParse(invalidUser);
      expect(result.success).toBe(false);
    });

    it('should reject a user with too long bio', () => {
      const invalidUser = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        displayName: 'Test User',
        bio: 'a'.repeat(161), // 161 characters, max is 160
        following: [],
        followers: [],
        createdAt: Date.now(),
      };

      const result = userSchema.safeParse(invalidUser);
      expect(result.success).toBe(false);
    });
  });

  describe('createUserSchema', () => {
    it('should validate valid create user input', () => {
      const validInput = {
        displayName: 'Test User',
        bio: 'This is a test bio',
      };

      const result = createUserSchema.safeParse(validInput);
      expect(result.success).toBe(true);
    });

    it('should validate input without bio', () => {
      const validInput = {
        displayName: 'Test User',
      };

      const result = createUserSchema.safeParse(validInput);
      expect(result.success).toBe(true);
    });

    it('should reject input with empty display name', () => {
      const invalidInput = {
        displayName: '',
        bio: 'This is a test bio',
      };

      const result = createUserSchema.safeParse(invalidInput);
      expect(result.success).toBe(false);
    });
  });

  describe('updateProfileSchema', () => {
    it('should validate valid update profile input', () => {
      const validInput = {
        displayName: 'Updated Name',
        bio: 'Updated bio',
        avatar: 'https://example.com/avatar.jpg',
      };

      const result = updateProfileSchema.safeParse(validInput);
      expect(result.success).toBe(true);
    });

    it('should validate partial update', () => {
      const validInput = {
        bio: 'Only updating bio',
      };

      const result = updateProfileSchema.safeParse(validInput);
      expect(result.success).toBe(true);
    });

    it('should reject input with too long display name', () => {
      const invalidInput = {
        displayName: 'a'.repeat(51), // 51 characters, max is 50
      };

      const result = updateProfileSchema.safeParse(invalidInput);
      expect(result.success).toBe(false);
    });
  });
});