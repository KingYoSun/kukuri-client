/**
 * ストレージサービス
 * 
 * ローカルストレージを使用したデータの永続化機能を提供します。
 * 将来的にはTauriのファイルシステムAPIを使用した実装に置き換える予定です。
 */

/**
 * データをローカルストレージに保存します
 */
export function saveData<T>(key: string, data: T): void {
  try {
    const serialized = JSON.stringify(data);
    localStorage.setItem(key, serialized);
  } catch (error) {
    console.error(`Error saving data for key ${key}:`, error);
  }
}

/**
 * データをローカルストレージから取得します
 */
export function getData<T>(key: string, defaultValue: T): T {
  try {
    const serialized = localStorage.getItem(key);
    if (serialized === null) {
      return defaultValue;
    }
    return JSON.parse(serialized) as T;
  } catch (error) {
    console.error(`Error retrieving data for key ${key}:`, error);
    return defaultValue;
  }
}

/**
 * データをローカルストレージから削除します
 */
export function removeData(key: string): void {
  try {
    localStorage.removeItem(key);
  } catch (error) {
    console.error(`Error removing data for key ${key}:`, error);
  }
}

/**
 * すべてのデータをローカルストレージから削除します
 */
export function clearAllData(): void {
  try {
    localStorage.clear();
  } catch (error) {
    console.error("Error clearing all data:", error);
  }
}

/**
 * 指定されたプレフィックスを持つすべてのキーを取得します
 */
export function getKeysWithPrefix(prefix: string): string[] {
  try {
    const keys: string[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && key.startsWith(prefix)) {
        keys.push(key);
      }
    }
    return keys;
  } catch (error) {
    console.error(`Error getting keys with prefix ${prefix}:`, error);
    return [];
  }
}