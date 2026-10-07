import { computed, ref } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'system';
export type ResolvedTheme = 'light' | 'dark';

const STORAGE_KEY = 'app-theme';
const DARK_QUERY = '(prefers-color-scheme: dark)';

/**
 * 主题模式存在 localStorage 而不是 store.json：
 * 它需要在首帧之前同步读到（见 index.html 的内联脚本），
 * 而 store.json 只能异步读，会造成一次主题闪烁。
 */
function readStoredMode(): ThemeMode {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw === 'light' || raw === 'dark' || raw === 'system') {
      return raw;
    }
  } catch {
    // localStorage 不可用时退回跟随系统
  }
  return 'system';
}

const mode = ref<ThemeMode>(readStoredMode());
const systemPrefersDark = ref(
  typeof window !== 'undefined' && window.matchMedia(DARK_QUERY).matches
);

const resolved = computed<ResolvedTheme>(() =>
  mode.value === 'system' ? (systemPrefersDark.value ? 'dark' : 'light') : mode.value
);

function apply(theme: ResolvedTheme) {
  document.documentElement.classList.toggle('dark', theme === 'dark');
}

/** 跟随系统时才需要监听；用一个常驻监听，状态本身很轻 */
if (typeof window !== 'undefined') {
  const media = window.matchMedia(DARK_QUERY);
  systemPrefersDark.value = media.matches;
  media.addEventListener('change', (event) => {
    systemPrefersDark.value = event.matches;
    if (mode.value === 'system') {
      apply(event.matches ? 'dark' : 'light');
    }
  });
}

/** 模块加载时对齐一次（index.html 已经pre-paint 设置过，这里做兜底） */
apply(resolved.value);

export function setThemeMode(next: ThemeMode) {
  mode.value = next;
  try {
    localStorage.setItem(STORAGE_KEY, next);
  } catch {
    // 存不进去也不影响本次会话
  }
  apply(resolved.value);
}

export function useTheme() {
  return {
    mode,
    resolved,
    setThemeMode,
  };
}
