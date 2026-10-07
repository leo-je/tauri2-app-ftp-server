<template>
  <div class="app-shell">
    <!-- 启动页自带一份同样的背景，这里等它退场后再挂载，避免两套网格和辉光同时合成 -->
    <AppBackground v-if="appReady" />

    <AppTitleBar />

    <div class="app-body">
      <AppSidebar />

      <main class="app-content" @contextmenu="disableContextMenu">
        <RouterView v-slot="{ Component, route }">
          <KeepAlive>
            <component :is="Component" :class="['page', isFirstVisit(route.name) ? 'stagger' : '']" />
          </KeepAlive>
        </RouterView>
      </main>
    </div>
  </div>

  <SplashScreen v-if="!appReady" @complete="onSplashComplete" />
</template>

<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router';
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { listen, emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import appStore from './store';
import AppTitleBar from './components/AppTitleBar.vue';
import AppSidebar from './components/AppSidebar.vue';
import AppBackground from './components/AppBackground.vue';
import SplashScreen from './components/SplashScreen.vue';

const { t } = useI18n();
const appWindow = getCurrentWindow();
const appReady = ref(false);
const route = useRoute();

/**
 * 入场动画只在每页「第一次看到」时播一次。
 * 页面被 KeepAlive 缓存，切回来时 DOM 会被重新插入，CSS 动画会跟着重播——
 * 每次切页都抖一下并不是想要的效果，所以这里记录已经看过哪些页。
 */
const visitedRoutes = ref(new Set<string>());

watch(
  () => route.name,
  (_to, from) => {
    if (typeof from === 'string') {
      visitedRoutes.value.add(from);
    }
  }
);

const isFirstVisit = (name: unknown) => typeof name === 'string' && !visitedRoutes.value.has(name);

/* 托盘菜单的启停是「一次点击、异步完成」，用一个待处理标记防止连点重复触发 */
const isTrayTogglePending = ref(false);
const TRAY_TOGGLE_TIMEOUT = 15000;
let trayToggleTimeout: ReturnType<typeof setTimeout> | null = null;
let unlistenTrayToggle: (() => void) | null = null;
let unlistenToggleFinished: (() => void) | null = null;

const clearTrayTogglePending = () => {
  isTrayTogglePending.value = false;
  if (trayToggleTimeout) {
    clearTimeout(trayToggleTimeout);
    trayToggleTimeout = null;
  }
};

const scheduleTrayToggleTimeout = () => {
  if (trayToggleTimeout) {
    clearTimeout(trayToggleTimeout);
  }

  trayToggleTimeout = setTimeout(() => {
    clearTrayTogglePending();
  }, TRAY_TOGGLE_TIMEOUT);
};

const initTrayMenuLanguage = async () => {
  try {
    await invoke('update_tray_menu_language', {
      isRunning: appStore.runtimeState.isServerRunning.value,
      text: {
        show: t('tray.show'),
        start: t('tray.start'),
        stop: t('tray.stop'),
        quit: t('tray.quit'),
        uptime: t('tray.uptime')
      }
    });
  } catch (e) {
    console.warn('Failed to init tray menu language:', e);
  }
};

onMounted(async () => {
  await initTrayMenuLanguage();

  unlistenTrayToggle = await listen('tray-toggle-ftp', async () => {
    if (isTrayTogglePending.value) {
      return;
    }

    isTrayTogglePending.value = true;
    scheduleTrayToggleTimeout();

    try {
      const isRunning = await invoke<boolean>('get_server_running');
      if (isRunning) {
        await emit('global-stop-ftp');
      } else {
        await emit('global-start-ftp');
      }
    } catch (e) {
      console.warn('Failed to toggle FTP from tray:', e);
      clearTrayTogglePending();
    }
  });

  unlistenToggleFinished = await listen('ftp-toggle-finished', () => {
    clearTrayTogglePending();
  });
});

onUnmounted(() => {
  clearTrayTogglePending();

  if (unlistenTrayToggle) {
    unlistenTrayToggle();
    unlistenTrayToggle = null;
  }

  if (unlistenToggleFinished) {
    unlistenToggleFinished();
    unlistenToggleFinished = null;
  }
});

function disableContextMenu(event: Event) {
  event.preventDefault();
}

const onSplashComplete = async () => {
  appReady.value = true;
  try {
    const hideOnStartup = await appStore.get('hideOnStartup');
    // 只有用户明确关掉了「启动时隐藏」才主动显示窗口
    if (hideOnStartup === false) {
      await appWindow.show();
    }
  } catch (e) {
    console.warn('Failed to check hide-on-startup setting:', e);
    await appWindow.show();
  }
};
</script>

<style scoped lang="scss">
.app-shell {
  position: relative;
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  overflow: hidden;
  background: var(--bg);
  /* 透明窗口：圆角 + 一圈发丝边，让窗口在桌面上有实体感 */
  border-radius: 12px;
  box-shadow: 0 0 0 1px var(--border);
}

.app-body {
  position: relative;
  z-index: 1;
  display: flex;
  flex: 1;
  min-height: 0;
}

.app-content {
  position: relative;
  z-index: 1;
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  overflow-x: hidden;
}

/* 页面容器由外壳统一给出，视图里不再各自处理宽度和内边距 */
.app-content :deep(.page) {
  max-width: 660px;
  margin: 0 auto;
  padding: var(--s-5) var(--s-5) var(--s-7);
}

@media (max-width: 700px) {
  .app-content :deep(.page) {
    padding: var(--s-4) var(--s-4) var(--s-6);
  }
}
</style>
