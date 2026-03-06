<template>
  <Transition name="splash-fade" @after-leave="onTransitionComplete">
    <div v-if="visible" class="splash-screen">
      <!-- 动态背景层 -->
      <div class="splash-bg">
        <!-- 渐变球体装饰 -->
        <div class="orb orb-1"></div>
        <div class="orb orb-2"></div>
        <div class="orb orb-3"></div>
        <!-- 网格背景 -->
        <div class="grid-overlay"></div>
      </div>

      <!-- 中央内容 -->
      <div class="splash-content">
        <!-- Logo 容器 -->
        <div class="logo-wrapper" :class="{ 'logo-ready': isReady }">
          <div class="logo-glow"></div>
          <div class="logo-pulse"></div>
          <div class="splash-logo">
            <AppLogo :size="72" />
          </div>
        </div>

        <!-- 欢迎语 -->
        <h1 class="splash-title" :class="{ 'title-ready': isReady }">
          {{ $t('splash.welcome') }}
        </h1>

        <!-- 版本号 -->
        <span class="splash-version" :class="{ 'version-ready': isReady }">
          v{{ version }}
        </span>

        <!-- 进度条 -->
        <div class="progress-container" :class="{ 'progress-ready': isReady }">
          <div class="progress-track">
            <div class="progress-bar" :style="{ width: `${progress}%` }"></div>
            <div class="progress-glow" :style="{ width: `${progress}%` }"></div>
          </div>
          <div class="progress-info">
            <span class="progress-status">{{ loadingStatus }}</span>
            <span class="progress-percent">{{ Math.round(progress) }}%</span>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import AppLogo from './AppLogo.vue';
import { getVersion } from '@tauri-apps/api/app';

const { t } = useI18n();
const visible = ref(true);
const version = ref('0.1.0');
const progress = ref(0);
const isReady = ref(false);

const emit = defineEmits<{
  complete: [];
}>();

// 加载状态文本
const loadingStatus = computed(() => {
  if (progress.value < 30) return t('splash.loading');
  if (progress.value < 60) return t('splash.loading') + '...';
  if (progress.value < 90) return t('splash.loading') + '..';
  return t('splash.ready');
});

onMounted(async () => {
  // 获取应用版本
  try {
    version.value = await getVersion();
  } catch {
    version.value = '0.1.0';
  }

  // 触发动画
  setTimeout(() => {
    isReady.value = true;
  }, 100);

  // 模拟进度加载
  const duration = 2000; // 2秒总时长
  const interval = 20; // 每20ms更新一次
  const steps = duration / interval;
  const increment = 100 / steps;

  let currentProgress = 0;
  const timer = setInterval(() => {
    // 非线性进度增长，前快后慢
    const remaining = 100 - currentProgress;
    const step = increment * (1 + remaining / 50);
    currentProgress += step;

    if (currentProgress >= 100) {
      currentProgress = 100;
      progress.value = 100;
      clearInterval(timer);

      // 进度完成后延迟隐藏
      setTimeout(() => {
        emit('complete');
        visible.value = false;
      }, 300);
    } else {
      progress.value = currentProgress;
    }
  }, interval);
});

const onTransitionComplete = () => {
  // 过渡动画完成后的回调
};
</script>

<style lang="scss" scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  overflow: hidden;
  border-radius: 12px;
}

/* 动态背景 */
.splash-bg {
  position: absolute;
  inset: 0;
  overflow: hidden;
}

/* 渐变球体 */
.orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.6;
}

.orb-1 {
  width: 400px;
  height: 400px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  top: -100px;
  right: -100px;
  animation: orbFloat1 15s ease-in-out infinite;
}

.orb-2 {
  width: 300px;
  height: 300px;
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  bottom: -50px;
  left: -50px;
  animation: orbFloat2 12s ease-in-out infinite;
}

.orb-3 {
  width: 200px;
  height: 200px;
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  animation: orbFloat3 18s ease-in-out infinite;
}

/* 网格背景 */
.grid-overlay {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
  background-size: 50px 50px;
  animation: gridMove 20s linear infinite;
}

/* 中央内容 */
.splash-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  z-index: 1;
  padding: 40px;
}

/* Logo 容器 */
.logo-wrapper {
  position: relative;
  width: 100px;
  height: 100px;
  opacity: 0;
  transform: scale(0.5) rotate(-180deg);
  transition: all 0.8s cubic-bezier(0.34, 1.56, 0.64, 1);

  &.logo-ready {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }
}

.logo-glow {
  position: absolute;
  inset: -10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
  border-radius: 24px;
  filter: blur(20px);
  opacity: 0.5;
  animation: glowPulse 2s ease-in-out infinite;
}

.logo-pulse {
  position: absolute;
  inset: 0;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  animation: logoPulse 2s ease-in-out infinite;
}

.splash-logo {
  position: relative;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, rgba(255, 255, 255, 0.05) 100%);
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  padding: 12px;
  overflow: hidden;

  :deep(.app-logo-img) {
    width: 100%;
    height: 100%;
    object-fit: contain;
    border-radius: 12px;
    animation: logoBreathe 3s ease-in-out infinite;
  }
}

/* 标题 */
.splash-title {
  font-size: 26px;
  font-weight: 700;
  color: white;
  letter-spacing: 1px;
  margin: 0;
  text-shadow: 0 2px 20px rgba(0, 0, 0, 0.3);
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.2s;

  &.title-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 版本号 */
.splash-version {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.6);
  font-weight: 500;
  letter-spacing: 1px;
  padding: 4px 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  backdrop-filter: blur(10px);
  opacity: 0;
  transform: translateY(10px);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.3s;

  &.version-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 进度条容器 */
.progress-container {
  width: 280px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.4s;

  &.progress-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

.progress-track {
  position: relative;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
  border-radius: 3px;
  transition: width 0.1s linear;
  position: relative;

  &::after {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 20px;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5));
    animation: progressShine 1s ease-in-out infinite;
  }
}

.progress-glow {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(102, 126, 234, 0.5), transparent);
  border-radius: 3px;
  filter: blur(4px);
  transition: width 0.1s linear;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.7);
}

.progress-status {
  font-weight: 500;
}

.progress-percent {
  font-weight: 600;
  color: rgba(255, 255, 255, 0.9);
  min-width: 35px;
  text-align: right;
}

/* 动画定义 */
@keyframes orbFloat1 {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  25% {
    transform: translate(30px, -30px) scale(1.1);
  }
  50% {
    transform: translate(-20px, 20px) scale(0.9);
  }
  75% {
    transform: translate(20px, 30px) scale(1.05);
  }
}

@keyframes orbFloat2 {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  33% {
    transform: translate(-40px, -20px) scale(1.15);
  }
  66% {
    transform: translate(30px, 10px) scale(0.85);
  }
}

@keyframes orbFloat3 {
  0%, 100% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.6;
  }
  50% {
    transform: translate(-50%, -50%) scale(1.3);
    opacity: 0.3;
  }
}

@keyframes gridMove {
  0% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(50px, 50px);
  }
}

@keyframes glowPulse {
  0%, 100% {
    opacity: 0.5;
    transform: scale(1);
  }
  50% {
    opacity: 0.8;
    transform: scale(1.1);
  }
}

@keyframes logoPulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.5;
  }
  50% {
    transform: scale(1.05);
    opacity: 0.8;
  }
}

@keyframes logoBreathe {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

@keyframes progressShine {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

/* 离开过渡动画 */
.splash-fade-leave-active {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.splash-fade-leave-to {
  opacity: 0;
  transform: scale(1.1);
  filter: blur(10px);
}
</style>
