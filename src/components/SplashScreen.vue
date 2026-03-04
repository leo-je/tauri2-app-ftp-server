<template>
  <div v-if="visible" class="splash-screen">
    <!-- 渐变背景装饰 -->
    <div class="splash-bg-decoration"></div>

    <!-- 中央内容 -->
    <div class="splash-content">
      <!-- Logo -->
      <div class="splash-logo">
        <AppLogo :size="64" />
      </div>

      <!-- 应用名称 -->
      <h1 class="splash-title">FTP Server</h1>

      <!-- 版本号 -->
      <span class="splash-version">v{{ version }}</span>

      <!-- 加载状态 -->
      <div class="splash-loading">
        <span class="loading-text">正在启动</span>
        <span class="loading-dots">
          <span class="dot"></span>
          <span class="dot"></span>
          <span class="dot"></span>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import AppLogo from './AppLogo.vue';
import { getVersion } from '@tauri-apps/api/app';

const visible = ref(true);
const version = ref('0.1.0');

const emit = defineEmits<{
  complete: [];
}>();

onMounted(async () => {
  // 获取应用版本
  try {
    version.value = await getVersion();
  } catch {
    version.value = '0.1.0';
  }

  // 2秒后先通知父组件显示主界面，再隐藏开屏
  setTimeout(() => {
    emit('complete');  // 先通知父组件
    visible.value = false;  // 再隐藏自己
  }, 2000);
});
</script>

<style lang="scss" scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: var(--gradient-bg);
  background-size: 400% 400%;
  animation: gradientShift 15s ease infinite;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  overflow: hidden;
  border-radius: 12px;
}

.splash-bg-decoration {
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
  animation: float 20s ease-in-out infinite;
  pointer-events: none;
}

.splash-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  animation: contentFadeIn 0.6s ease;
  z-index: 1;
}

.splash-logo {
  width: 80px;
  height: 80px;
  background: rgba(255, 255, 255, 0.15);
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  padding: 8px;

  :deep(.app-logo-img) {
    width: 100%;
    height: 100%;
    object-fit: contain;
    border-radius: 12px;
  }
}

.splash-title {
  font-size: 28px;
  font-weight: 700;
  color: white;
  letter-spacing: 1px;
  margin: 0;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.splash-version {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.7);
  font-weight: 500;
  letter-spacing: 0.5px;
}

.splash-loading {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.loading-text {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.8);
  font-weight: 500;
}

.loading-dots {
  display: flex;
  gap: 4px;

  .dot {
    width: 6px;
    height: 6px;
    background: rgba(255, 255, 255, 0.8);
    border-radius: 50%;
    animation: dotPulse 1.4s ease-in-out infinite;

    &:nth-child(2) {
      animation-delay: 0.2s;
    }

    &:nth-child(3) {
      animation-delay: 0.4s;
    }
  }
}

/* 渐变背景动画 */
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

/* 背景装饰浮动动画 */
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

/* 内容淡入动画 */
@keyframes contentFadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 加载点脉冲动画 */
@keyframes dotPulse {
  0%, 100% {
    opacity: 0.4;
    transform: scale(0.8);
  }
  50% {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
