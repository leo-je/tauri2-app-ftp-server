<template>
  <el-container class="app-container">
    <!-- 自定义标题栏 -->
    <div class="title-bar" data-tauri-drag-region>
      <div class="title-bar-content" data-tauri-drag-region>
        <div class="app-logo" data-tauri-drag-region>
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <span class="app-title" data-tauri-drag-region>FTP Server</span>
      </div>
      <div class="window-controls">
        <div class="control-btn minimize" @click="minimizeWindow" title="最小化">
          <svg viewBox="0 0 24 24" fill="none">
            <path d="M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </div>
        <div class="control-btn close" @click="closeWindow" title="关闭">
          <svg viewBox="0 0 24 24" fill="none">
            <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
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

const appWindow = getCurrentWindow();

function disableContextMenu(event: any) {
   event.preventDefault();
}

const minimizeWindow = () => {
  appWindow.minimize();
}

const closeWindow = () => {
  appWindow.close();
}
</script>

<style lang="scss" scoped>
.app-container {
  height: 100%;
  width: 100%;
  background: var(--gradient-bg);
  background-size: 400% 400%;
  animation: gradientShift 15s ease infinite;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;

  &::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
    animation: float 20s ease-in-out infinite;
    pointer-events: none;
  }
}

/* 自定义标题栏 */
.title-bar {
  height: 36px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.95) 0%, rgba(118, 75, 162, 0.95) 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 8px 0 12px;
  flex-shrink: 0;
  z-index: 1000;
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.title-bar-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-logo {
  width: 18px;
  height: 18px;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;

  svg {
    width: 100%;
    height: 100%;
  }
}

.app-title {
  font-size: 13px;
  font-weight: 600;
  color: white;
  letter-spacing: 0.5px;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 2px;
}

.control-btn {
  width: 36px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: rgba(255, 255, 255, 0.9);

  svg {
    width: 14px;
    height: 14px;
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

@keyframes float {
  0%, 100% {
    transform: translate(0, 0) rotate(0deg);
  }
  33% {
    transform: translate(30px, -30px) rotate(120deg);
  }
  66% {
    transform: translate(-20px, 20px) rotate(240deg);
  }
}

.index-main {
  color: var(--el-text-color-primary);
  flex: 1;
  width: 100%;
  padding: 0;
  overflow: auto;
  position: relative;
  z-index: 1;
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
