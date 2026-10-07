import { Store } from '@tauri-apps/plugin-store';
import { ref } from 'vue';

export interface UserItem {
  username: string;
  password: string;
  fileAuth: string;
}

export interface StoreValue {
  selected?: string;
  isAnonymous?: boolean;
  fileauth?: string;
  tableData?: UserItem[];
  hideOnStartup?: boolean;
  /**
   * IP 黑名单的持久化副本。
   * 后端那份是内存态（进程重启即空），启动时由前端把它推回去，
   * 这样黑名单才真的能跨重启生效。
   */
  blacklistIps?: string[];
}

let store: Store;

const getStore = async () => {
  if (!store) {
    store = await Store.load('store.json');
  }
  return store;
};

export const get = async <K extends keyof StoreValue>(key: K): Promise<StoreValue[K] | undefined> => {
  const s = await getStore();
  return s.get(key) as Promise<StoreValue[K] | undefined>;
};

export const set = async <K extends keyof StoreValue>(key: K, value: StoreValue[K]): Promise<void> => {
  const s = await getStore();
  await s.set(key, value);
  await s.save();
};

export const runtimeState = {
  isServerRunning: ref(false),
};

const appStore = {
  get,
  set,
  runtimeState,
};

export default appStore;
