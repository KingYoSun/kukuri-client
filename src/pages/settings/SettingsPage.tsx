import React, { useState } from 'react';
import { useAuthStore } from '@/stores/auth-store';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useTheme } from '@/components/theme-provider';
import { toast } from '@/hooks/use-toast';
import { invoke } from '@tauri-apps/api/core';

const SettingsPage: React.FC = () => {
  const { user, logout } = useAuthStore();
  const { theme, setTheme } = useTheme();
  const [relayUrl, setRelayUrl] = useState('');
  const [isAddingRelay, setIsAddingRelay] = useState(false);
  const [relays, setRelays] = useState<string[]>([
    'https://relay.kukuri.network',
    'https://relay.example.com'
  ]);

  const handleAddRelay = (e: React.FormEvent) => {
    e.preventDefault();
    if (!relayUrl.trim()) return;

    // Validate URL
    try {
      new URL(relayUrl);
      setRelays([...relays, relayUrl]);
      setRelayUrl('');
      setIsAddingRelay(false);
      toast({
        title: 'Relay added',
        description: 'The relay has been added successfully',
      });
    } catch (error) {
      toast({
        variant: 'destructive',
        title: 'Invalid URL',
        description: 'Please enter a valid URL',
      });
    }
  };

  const handleRemoveRelay = (url: string) => {
    setRelays(relays.filter(relay => relay !== url));
    toast({
      title: 'Relay removed',
      description: 'The relay has been removed successfully',
    });
  };

  const handleExportData = async () => {
    try {
      // This would be a Tauri command to export user data
      await invoke('export_user_data', { userId: user?.id });
      toast({
        title: 'Data exported',
        description: 'Your data has been exported successfully',
      });
    } catch (error) {
      toast({
        variant: 'destructive',
        title: 'Export failed',
        description: error instanceof Error ? error.message : 'Failed to export data',
      });
    }
  };

  return (
    <div className="container mx-auto max-w-3xl">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      
      <div className="space-y-6">
        <div className="bg-card rounded-lg p-6 shadow-sm">
          <h2 className="text-xl font-semibold mb-4">Appearance</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-2">Theme</label>
              <div className="flex space-x-2">
                <Button
                  variant={theme === 'light' ? 'default' : 'outline'}
                  onClick={() => setTheme('light')}
                >
                  Light
                </Button>
                <Button
                  variant={theme === 'dark' ? 'default' : 'outline'}
                  onClick={() => setTheme('dark')}
                >
                  Dark
                </Button>
                <Button
                  variant={theme === 'system' ? 'default' : 'outline'}
                  onClick={() => setTheme('system')}
                >
                  System
                </Button>
              </div>
            </div>
          </div>
        </div>
        
        <div className="bg-card rounded-lg p-6 shadow-sm">
          <h2 className="text-xl font-semibold mb-4">Network</h2>
          <div className="space-y-4">
            <div>
              <div className="flex justify-between items-center mb-2">
                <label className="block text-sm font-medium">Relays</label>
                {!isAddingRelay && (
                  <Button variant="outline" size="sm" onClick={() => setIsAddingRelay(true)}>
                    Add Relay
                  </Button>
                )}
              </div>
              
              {isAddingRelay && (
                <form onSubmit={handleAddRelay} className="mb-4 flex space-x-2">
                  <Input
                    placeholder="https://relay.example.com"
                    value={relayUrl}
                    onChange={(e) => setRelayUrl(e.target.value)}
                    className="flex-1"
                  />
                  <Button type="submit">Add</Button>
                  <Button type="button" variant="outline" onClick={() => setIsAddingRelay(false)}>
                    Cancel
                  </Button>
                </form>
              )}
              
              <div className="space-y-2">
                {relays.length === 0 ? (
                  <p className="text-sm text-muted-foreground">No relays configured</p>
                ) : (
                  relays.map((relay) => (
                    <div key={relay} className="flex justify-between items-center p-2 bg-background rounded">
                      <span className="text-sm">{relay}</span>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleRemoveRelay(relay)}
                        className="text-destructive hover:text-destructive/90 hover:bg-destructive/10"
                      >
                        Remove
                      </Button>
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>
        </div>
        
        <div className="bg-card rounded-lg p-6 shadow-sm">
          <h2 className="text-xl font-semibold mb-4">Data</h2>
          <div className="space-y-4">
            <Button variant="outline" onClick={handleExportData}>
              Export Your Data
            </Button>
          </div>
        </div>
        
        <div className="bg-card rounded-lg p-6 shadow-sm">
          <h2 className="text-xl font-semibold mb-4">Account</h2>
          <div className="space-y-4">
            <Button
              variant="destructive"
              onClick={logout}
            >
              Logout
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsPage;