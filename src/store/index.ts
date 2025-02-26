import { Store } from '@tauri-apps/plugin-store';
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

export const set = async (key: string, value: any) => {
    const store = await getStore();
    store.set(key, value);
    await store.save();
}

const appStore = {
    get,
    set
}

export default appStore;