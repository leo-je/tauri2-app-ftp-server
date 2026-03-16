import { Store } from '@tauri-apps/plugin-store';
import { ref } from 'vue';

let store: Store;//await Store.load('store.json');

const getStore = async () => {
    if (!store) {
        console.log('store not loaded');
        store = await Store.load('store.json');
    }
    return store;
}

export const get = async (key: string) => {
    const store = await getStore();
    return store.get(key);
}

export const set = async (key: string, value: unknown) => {
    const store = await getStore();
    await store.set(key, value);
    await store.save();
}

// 运行时状态（不持久化）
export const runtimeState = {
    isServerRunning: ref(false)
};

const appStore = {
    get,
    set,
    runtimeState
}

export default appStore;