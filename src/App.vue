<template>
  <el-container class="app-container">
    <!-- 自定义标题栏 -->
    <div class="title-bar" data-tauri-drag-region :class="{ 'is-macos': isMacos }">
      <!-- macOS 风格控制按钮 -->
      <div v-if="isMacos" class="window-controls macos-controls">
        <div class="macos-btn close" @click="closeWindow" title="关闭"></div>
        <div class="macos-btn minimize" @click="minimizeWindow" title="最小化"></div>
      </div>

      <div class="title-bar-content" data-tauri-drag-region>
        <div class="app-logo" data-tauri-drag-region>
          <SvgIcon name="server" :size="20" />
        </div>
        <span class="app-title" data-tauri-drag-region>FTP Server</span>
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

const appWindow = getCurrentWindow();
const isMacos = ref(false);

onMounted(async () => {
  isMacos.value = await platform() === 'macos';
});

function disableContextMenu(_event: MouseEvent) {
  //  event.preventDefault();
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
  height: 40px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.95) 0%, rgba(118, 75, 162, 0.95) 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 10px 0 14px;
  flex-shrink: 0;
  z-index: 1000;
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);

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

  svg {
    width: 100%;
    height: 100%;
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
