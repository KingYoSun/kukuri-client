import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useAuthStore } from '@/stores/auth-store';
import { toast } from '@/hooks/use-toast';

type AuthMode = 'login' | 'register';

const AuthPage: React.FC = () => {
  const [mode, setMode] = useState<AuthMode>('login');
  const [displayName, setDisplayName] = useState('');
  const [bio, setBio] = useState('');
  const [existingUsers, setExistingUsers] = useState<{ id: string; displayName: string }[]>([]);
  const [selectedUserId, setSelectedUserId] = useState<string | null>(null);
  const { createUser, signIn, isLoading } = useAuthStore();

  // Fetch existing users on component mount
  useEffect(() => {
    const fetchExistingUsers = async () => {
      try {
        // This is a mock implementation - in a real app, we would have a command to list users
        // For now, we'll just check if there are any user keys in the app data directory
        const users = await invoke('list_users');
        setExistingUsers(users as { id: string; displayName: string }[]);
      } catch (error) {
        console.error('Error fetching existing users:', error);
        // If we can't fetch users, we'll assume there are none and show the register form
        setMode('register');
      }
    };

    fetchExistingUsers();
  }, []);

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!displayName.trim()) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'Display name is required',
      });
      return;
    }

    await createUser(displayName, bio);
  };

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedUserId) {
      toast({
        variant: 'destructive',
        title: 'Error',
        description: 'Please select a user',
      });
      return;
    }

    await signIn(selectedUserId);
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-background">
      <div className="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
        <div className="flex flex-col space-y-2 text-center">
          <h1 className="text-2xl font-semibold tracking-tight">Kukuri</h1>
          <p className="text-sm text-muted-foreground">
            Choose your connections, control your network
          </p>
        </div>
        <div className="grid gap-6">
          {mode === 'register' ? (
            <form onSubmit={handleRegister}>
              <div className="grid gap-4">
                <div className="grid gap-2">
                  <Input
                    id="displayName"
                    placeholder="Display Name"
                    value={displayName}
                    onChange={(e) => setDisplayName(e.target.value)}
                    disabled={isLoading}
                    required
                  />
                </div>
                <div className="grid gap-2">
                  <Input
                    id="bio"
                    placeholder="Bio (optional)"
                    value={bio}
                    onChange={(e) => setBio(e.target.value)}
                    disabled={isLoading}
                  />
                </div>
                <Button type="submit" disabled={isLoading}>
                  {isLoading ? 'Creating Account...' : 'Create Account'}
                </Button>
              </div>
            </form>
          ) : (
            <form onSubmit={handleLogin}>
              <div className="grid gap-4">
                <div className="grid gap-2">
                  {existingUsers.length > 0 ? (
                    <div className="space-y-4">
                      <p className="text-sm text-center">Select your profile:</p>
                      <div className="grid gap-2">
                        {existingUsers.map((user) => (
                          <Button
                            key={user.id}
                            type="button"
                            variant={selectedUserId === user.id ? 'default' : 'outline'}
                            className="w-full justify-start"
                            onClick={() => setSelectedUserId(user.id)}
                          >
                            {user.displayName}
                          </Button>
                        ))}
                      </div>
                    </div>
                  ) : (
                    <p className="text-sm text-center">No existing profiles found.</p>
                  )}
                </div>
                {existingUsers.length > 0 && (
                  <Button type="submit" disabled={isLoading || !selectedUserId}>
                    {isLoading ? 'Signing In...' : 'Sign In'}
                  </Button>
                )}
              </div>
            </form>
          )}
        </div>
        <div className="text-center text-sm">
          {mode === 'login' ? (
            <Button variant="link" onClick={() => setMode('register')}>
              Create a new account
            </Button>
          ) : (
            <Button variant="link" onClick={() => setMode('login')}>
              Sign in with existing account
            </Button>
          )}
        </div>
      </div>
    </div>
  );
};

export default AuthPage;