<template>
  <SplashScreen v-if="!appReady" @complete="appReady = true" />
  <el-container class="app-container">
    <AppBackground v-if="appReady" />
    <!-- 自定义标题栏 -->
    <div class="title-bar" data-tauri-drag-region :class="{ 'is-macos': isMacos }">
      <!-- macOS 风格控制按钮 -->
      <div v-if="isMacos" class="window-controls macos-controls">
        <div class="macos-btn close" @click="closeWindow" title="关闭"></div>
        <div class="macos-btn minimize" @click="minimizeWindow" title="最小化"></div>
      </div>

      <div class="title-bar-content" data-tauri-drag-region>
        <div class="app-logo" data-tauri-drag-region>
          <AppLogo :size="20" />
        </div>
        <span class="app-title" data-tauri-drag-region>{{ $t('app.name') }}</span>
      </div>

      <!-- 语言切换按钮 - 悬浮于标题栏下方 -->
      <div class="floating-lang-switcher" v-if="appReady">
        <LanguageSwitcher />
      </div>

      <!-- Windows/Linux 风格控制按钮 -->
      <div v-if="!isMacos" class="window-controls">
        <div class="control-btn minimize" @click="minimizeWindow" title="最小化">
          <SvgIcon name="minimize" :size="16" />
        </div>
        <div class="control-btn close" @click="closeWindow" title="关闭">
          <SvgIcon name="close" :size="16" />
        </div>
      </div>
    </div>

    <el-main class="index-main" @contextmenu="disableContextMenu">
      <RouterView />
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ElMain, ElContainer } from 'element-plus';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { platform } from '@tauri-apps/plugin-os';
import { ref, onMounted } from 'vue';
import { SvgIcon } from './components/icons';
import AppLogo from './components/AppLogo.vue';
import SplashScreen from './components/SplashScreen.vue';
import LanguageSwitcher from './components/LanguageSwitcher.vue';
import AppBackground from './components/AppBackground.vue';
import { listen, emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

const appWindow = getCurrentWindow();
const isMacos = ref(false);
const appReady = ref(false);

onMounted(async () => {
  isMacos.value = await platform() === 'macos';

  // 监听托盘菜单事件 - 切换 FTP 服务
  await listen('tray-toggle-ftp', async () => {
    console.log('托盘事件：切换 FTP 服务');
    // 从后端获取当前状态
    const isRunning = await invoke<boolean>('get_server_running');
    // 根据当前状态触发相应事件
    if (isRunning) {
      await emit('global-stop-ftp');
    } else {
      await emit('global-start-ftp');
    }
  });
});

function disableContextMenu() {
  //event.preventDefault();
}

const minimizeWindow = () => {
  appWindow.minimize();
}

const closeWindow = () => {
  // 隐藏窗口而不是关闭应用
  appWindow.hide();
}
</script>

<style lang="scss" scoped>
.app-container {
  height: 100%;
  width: 100%;
  background: var(--gradient-bg);
  background-size: 180% 180%;
  animation: gradientShift 22s ease infinite;
  position: relative;
  overflow: hidden;
  isolation: isolate;
  display: flex;
  flex-direction: column;
  border-radius: 12px;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.1), 0 20px 40px rgba(0, 0, 0, 0.15);
}

/* 自定义标题栏 */
.title-bar {
  height: 40px;
  background: linear-gradient(135deg, rgba(24, 39, 78, 0.7) 0%, rgba(69, 46, 116, 0.66) 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 10px 0 14px;
  flex-shrink: 0;
  z-index: 3;
  backdrop-filter: blur(18px) saturate(120%);
  border-bottom: 1px solid rgba(255, 255, 255, 0.12);
  box-shadow: 0 6px 18px rgba(6, 10, 24, 0.14);

  /* macOS 风格 */
  &.is-macos {
    padding: 0 14px;

    .title-bar-content {
      position: absolute;
      left: 50%;
      transform: translateX(-50%);
    }
  }
}

.title-bar-content {
  display: flex;
  align-items: center;
  gap: 10px;
}

.app-logo {
  width: 20px;
  height: 20px;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  border-radius: 4px;

  :deep(.app-logo-img) {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
}

.app-title {
  font-size: 14px;
  font-weight: 600;
  color: white;
  letter-spacing: 0.5px;
}

/* macOS 风格控制按钮 */
.macos-controls {
  display: flex;
  align-items: center;
  gap: 10px;
  z-index: 10;
}

.macos-btn {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.15s ease;
  position: relative;

  &::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  &.close {
    background: #ff5f57;
    border: 1px solid #e0443e;

    &:hover::after {
      content: '×';
      font-size: 12px;
      font-weight: bold;
      color: #4d0000;
      opacity: 1;
    }
  }

  &.minimize {
    background: #febc2e;
    border: 1px solid #dea123;

    &:hover::after {
      content: '−';
      font-size: 12px;
      font-weight: bold;
      color: #995700;
      opacity: 1;
    }
  }
}

/* Windows/Linux 风格控制按钮 */
.window-controls {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 悬浮语言切换按钮 */
.floating-lang-switcher {
  position: fixed;
  top: 48px;
  right: 16px;
  z-index: 4;
}

.control-btn {
  width: 40px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: rgba(255, 255, 255, 0.9);

  svg {
    width: 16px;
    height: 16px;
  }

  &:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  &.close:hover {
    background: rgba(245, 101, 101, 0.9);
  }
}

@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

.index-main {
  color: var(--el-text-color-primary);
  flex: 1;
  width: 100%;
  padding: 0;
  overflow: auto;
  position: relative;
  z-index: 2;
}

* {
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>

<style lang="scss">
/* 全局样式 - 拖拽区域 */
*[data-tauri-drag-region] {
  -webkit-app-region: drag;
  app-region: drag;
}
</style>
