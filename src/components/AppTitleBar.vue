<template>
  <header class="titlebar" data-tauri-drag-region :class="{ 'is-macos': isMacos }">
    <div class="slot slot-start">
      <div v-if="isMacos" class="macos-controls">
        <button
          type="button"
          class="macos-dot close"
          :title="$t('window.close')"
          :aria-label="$t('window.close')"
          @click="closeWindow"
        ></button>
        <button
          type="button"
          class="macos-dot minimize"
          :title="$t('window.minimize')"
          :aria-label="$t('window.minimize')"
          @click="minimizeWindow"
        ></button>
      </div>
    </div>

    <div class="title" data-tauri-drag-region>
      <AppLogo :size="15" class="title-logo" />
      <span class="title-text">{{ $t('app.name') }}</span>
    </div>

    <div class="slot slot-end">
      <div v-if="!isMacos" class="win-controls">
        <button
          type="button"
          class="win-btn"
          :title="$t('window.minimize')"
          :aria-label="$t('window.minimize')"
          @click="minimizeWindow"
        >
          <SvgIcon name="minimize" :size="14" />
        </button>
        <button
          type="button"
          class="win-btn close"
          :title="$t('window.close')"
          :aria-label="$t('window.close')"
          @click="closeWindow"
        >
          <SvgIcon name="close" :size="14" />
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { platform } from '@tauri-apps/plugin-os';
import { invoke } from '@tauri-apps/api/core';
import { SvgIcon } from './icons';
import AppLogo from './AppLogo.vue';

const appWindow = getCurrentWindow();
const isMacos = ref(false);

onMounted(async () => {
  isMacos.value = (await platform()) === 'macos';
});

const minimizeWindow = () => {
  appWindow.minimize();
};

/** 关闭 = 隐藏窗口并保留托盘/Dock，符合常驻托盘应用的行为 */
const closeWindow = () => {
  invoke('hide_main_window');
};
</script>

<style scoped lang="scss">
.titlebar {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  height: var(--titlebar-h);
  flex-shrink: 0;
  padding: 0 var(--s-2) 0 var(--s-3);
  background: var(--chrome);
  border-bottom: 1px solid var(--border);
  z-index: 3;
}

.slot {
  display: flex;
  align-items: center;
  min-width: 0;
}

.slot-end {
  justify-content: flex-end;
}

.title {
  display: flex;
  align-items: center;
  gap: 7px;
  padding: 0 var(--s-2);
}

.title-logo {
  opacity: 0.9;
  border-radius: var(--r-xs);
}

.title-text {
  font-size: var(--fs-md);
  font-weight: 600;
  letter-spacing: 0.005em;
  color: var(--text-2);
  white-space: nowrap;
}

/* ---------- macOS 自绘控制点 ---------- */
.macos-controls {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  padding-left: 2px;
}

.macos-dot {
  position: relative;
  width: 12px;
  height: 12px;
  padding: 0;
  border: 0;
  border-radius: var(--r-full);
  cursor: pointer;
  transition: filter var(--dur-1) var(--ease-standard);

  &::after {
    content: '';
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    font-weight: 700;
    line-height: 1;
    opacity: 0;
    transition: opacity var(--dur-1) var(--ease-standard);
  }

  &:hover::after {
    opacity: 0.62;
  }

  &:active {
    filter: brightness(0.85);
  }

  &.close {
    background: #ff5f57;
    color: #6b0d06;

    &::after {
      content: '✕';
    }
  }

  &.minimize {
    background: #febc2e;
    color: #8a5200;

    &::after {
      content: '−';
    }
  }
}

/* ---------- Windows / Linux 控制按钮 ---------- */
.win-controls {
  display: flex;
  align-items: center;
  gap: 2px;
}

.win-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 34px;
  height: 26px;
  border: 0;
  border-radius: var(--r-sm);
  background: transparent;
  color: var(--text-2);
  cursor: pointer;
  transition: background var(--dur-1) var(--ease-standard),
    color var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--chrome-hover);
    color: var(--text);
  }

  &.close:hover {
    background: var(--danger);
    color: #fff;
  }
}
</style>
