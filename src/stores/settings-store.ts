import { create } from 'zustand';
import { Settings, UpdateSettingsInput } from '@/models/settings';
import { getSettings, updateSettings } from '@/services/settings-service';
import { toast } from '@/hooks/use-toast';
import { THEMES } from '@/lib/constants';

interface SettingsState {
  settings: Settings | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchSettings: (userId?: string) => Promise<void>;
  updateSettings: (userId: string | undefined, data: UpdateSettingsInput) => Promise<boolean>;
  setTheme: (theme: typeof THEMES[keyof typeof THEMES]) => Promise<boolean>;
  setLanguage: (language: string) => Promise<boolean>;
  toggleNotifications: () => Promise<boolean>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  isLoading: false,
  error: null,

  fetchSettings: async (userId?: string) => {
    set({ isLoading: true, error: null });
    try {
      const settings = await getSettings(userId);
      set({ settings, isLoading: false });
    } catch (error) {
      console.error('Error fetching settings:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to fetch settings',
      });
    }
  },

  updateSettings: async (userId: string | undefined, data: UpdateSettingsInput): Promise<boolean> => {
    set({ isLoading: true, error: null });
    try {
      const result = await updateSettings(userId, data);
      
      if (result.success) {
        // 設定を再取得
        await get().fetchSettings(userId);
        set({ isLoading: false });
        toast({
          title: '設定が更新されました',
          description: '設定が正常に更新されました',
        });
        return true;
      } else {
        throw new Error(result.message || 'Failed to update settings');
      }
    } catch (error) {
      console.error('Error updating settings:', error);
      set({ 
        error: error instanceof Error ? error.message : 'An unknown error occurred', 
        isLoading: false 
      });
      toast({
        variant: 'destructive',
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to update settings',
      });
      return false;
    }
  },

  setTheme: async (theme: typeof THEMES[keyof typeof THEMES]): Promise<boolean> => {
    const { settings } = get();
    if (!settings) return false;

    return get().updateSettings(settings.userId, { theme });
  },

  setLanguage: async (language: string): Promise<boolean> => {
    const { settings } = get();
    if (!settings) return false;

    return get().updateSettings(settings.userId, { language });
  },

  toggleNotifications: async (): Promise<boolean> => {
    const { settings } = get();
    if (!settings) return false;

    return get().updateSettings(settings.userId, { 
      notifications: !settings.notifications 
    });
  },
}));