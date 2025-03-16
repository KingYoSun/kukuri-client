import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { saveData, getData, removeData, clearAllData, getKeysWithPrefix } from '@/services/storage-service';

describe('Storage Service', () => {
  // モックのローカルストレージを設定
  const localStorageMock = (() => {
    let store: Record<string, string> = {};
    return {
      getItem: vi.fn((key: string) => {
        return store[key] || null;
      }),
      setItem: vi.fn((key: string, value: string) => {
        store[key] = value;
      }),
      removeItem: vi.fn((key: string) => {
        delete store[key];
      }),
      clear: vi.fn(() => {
        store = {};
      }),
      key: vi.fn((index: number) => {
        return Object.keys(store)[index] || null;
      }),
      length: 0,
      get length() {
        return Object.keys(store).length;
      },
    };
  })();

  // テスト前にローカルストレージをモックに置き換え
  beforeEach(() => {
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true,
    });
  });

  // テスト後にモックをリセット
  afterEach(() => {
    vi.clearAllMocks();
    localStorageMock.clear();
  });

  describe('saveData', () => {
    it('should save data to localStorage', () => {
      const testData = { name: 'Test User', age: 30 };
      saveData('testKey', testData);

      expect(localStorageMock.setItem).toHaveBeenCalledWith('testKey', JSON.stringify(testData));
    });

    it('should handle errors when saving data', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const testData = { name: 'Test User', age: 30 };

      // エラーをシミュレート
      localStorageMock.setItem.mockImplementationOnce(() => {
        throw new Error('Storage error');
      });

      saveData('testKey', testData);

      expect(consoleSpy).toHaveBeenCalled();
      consoleSpy.mockRestore();
    });
  });

  describe('getData', () => {
    it('should retrieve data from localStorage', () => {
      const testData = { name: 'Test User', age: 30 };
      localStorageMock.getItem.mockReturnValueOnce(JSON.stringify(testData));

      const result = getData('testKey', {});

      expect(localStorageMock.getItem).toHaveBeenCalledWith('testKey');
      expect(result).toEqual(testData);
    });

    it('should return default value if key does not exist', () => {
      const defaultValue = { name: 'Default', age: 0 };
      localStorageMock.getItem.mockReturnValueOnce(null);

      const result = getData('nonExistentKey', defaultValue);

      expect(result).toEqual(defaultValue);
    });

    it('should handle parsing errors', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const defaultValue = { name: 'Default', age: 0 };

      // 無効なJSONをシミュレート
      localStorageMock.getItem.mockReturnValueOnce('invalid json');

      const result = getData('testKey', defaultValue);

      expect(consoleSpy).toHaveBeenCalled();
      expect(result).toEqual(defaultValue);
      consoleSpy.mockRestore();
    });
  });

  describe('removeData', () => {
    it('should remove data from localStorage', () => {
      removeData('testKey');

      expect(localStorageMock.removeItem).toHaveBeenCalledWith('testKey');
    });

    it('should handle errors when removing data', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      // エラーをシミュレート
      localStorageMock.removeItem.mockImplementationOnce(() => {
        throw new Error('Storage error');
      });

      removeData('testKey');

      expect(consoleSpy).toHaveBeenCalled();
      consoleSpy.mockRestore();
    });
  });

  describe('clearAllData', () => {
    it('should clear all data from localStorage', () => {
      clearAllData();

      expect(localStorageMock.clear).toHaveBeenCalled();
    });

    it('should handle errors when clearing data', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      // エラーをシミュレート
      localStorageMock.clear.mockImplementationOnce(() => {
        throw new Error('Storage error');
      });

      clearAllData();

      expect(consoleSpy).toHaveBeenCalled();
      consoleSpy.mockRestore();
    });
  });

  describe('getKeysWithPrefix', () => {
    it('should return keys with the specified prefix', () => {
      // モックのストレージにいくつかのキーを設定
      Object.defineProperty(localStorageMock, 'length', { value: 3 });
      localStorageMock.key.mockImplementation((index: number) => {
        const keys = ['prefix:key1', 'prefix:key2', 'otherKey'];
        return keys[index] || null;
      });

      const result = getKeysWithPrefix('prefix:');

      expect(result).toEqual(['prefix:key1', 'prefix:key2']);
    });

    it('should handle errors when getting keys', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      // エラーをシミュレート
      localStorageMock.key.mockImplementationOnce(() => {
        throw new Error('Storage error');
      });

      const result = getKeysWithPrefix('prefix:');

      expect(consoleSpy).toHaveBeenCalled();
      expect(result).toEqual([]);
      consoleSpy.mockRestore();
    });
  });
});