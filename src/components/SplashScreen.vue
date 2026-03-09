<template>
  <Transition name="splash-fade" @after-leave="onTransitionComplete">
    <div v-if="visible" class="splash-screen">
      <!-- 粒子动画背景 -->
      <canvas
        v-if="!isReducedMotion"
        ref="particleCanvas"
        class="particle-canvas"
      ></canvas>

      <!-- 渐变背景覆盖层 -->
      <div class="gradient-overlay"></div>

      <!-- 中央内容 -->
      <div class="splash-content" :class="{ 'content-ready': isReady }">
        <!-- Logo -->
        <SplashLogo :size="72" :ready="isReady" />

        <!-- 欢迎语 -->
        <h1 class="splash-title" :class="{ 'title-ready': isReady }">
          {{ $t('splash.welcome') }}
        </h1>

        <!-- 版本号 -->
        <span class="splash-version" :class="{ 'version-ready': isReady }">
          v{{ version }}
        </span>

        <!-- 初始化步骤 -->
        <div class="steps-wrapper" :class="{ 'steps-ready': isReady }">
          <InitSteps
            :steps="initSteps"
            :current-index="currentStepIndex"
            :progress="overallProgress"
          />
          <ProgressBar :progress="overallProgress" />
        </div>
      </div>

      <!-- 错误提示 -->
      <ErrorMessage
        :title="errorTitle"
        :message="errorMessage"
        :visible="hasError"
        @retry="retryInitialization"
        @skip="skipInitialization"
      />

      <!-- 键盘快捷键提示 -->
      <div v-if="hasError" class="keyboard-hints">
        <span class="hint">{{ $t('splash.error.retry') }}: R</span>
        <span class="hint">{{ $t('splash.error.skip') }}: ESC</span>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { getVersion } from '@tauri-apps/api/app';

// 子组件
import SplashLogo from './splash/components/SplashLogo.vue';
import InitSteps from './splash/components/InitSteps.vue';
import ProgressBar from './splash/components/ProgressBar.vue';
import ErrorMessage from './splash/components/ErrorMessage.vue';

// Composables
import { useInitialization } from './splash/composables/useInitialization';
import { useParticleAnimation } from './splash/composables/useParticleAnimation';

const { t } = useI18n();

// 可见性状态
const visible = ref(true);
const isReady = ref(false);
const version = ref('0.1.0');
const particleCanvas = ref<HTMLCanvasElement | null>(null);

// 使用初始化 composable
const {
  initSteps,
  currentStepIndex,
  hasError,
  errorMessage,
  overallProgress,
  runInitialization,
  retryInitialization,
  skipInitialization
} = useInitialization({
  t,
  onComplete: () => {
    // 初始化完成，延迟后关闭启动页面
    setTimeout(() => {
      emit('complete');
      visible.value = false;
    }, 800);
  },
  onError: (message) => {
    console.error('Initialization error:', message);
  }
});

// 使用粒子动画 composable
const { isReducedMotion, start: startParticles, stop: stopParticles } = useParticleAnimation(
  particleCanvas,
  {
    maxParticles: 80,
    connectionDistance: 120,
    enableMouseInteraction: true
  }
);

// 计算错误标题
const errorTitle = computed(() => t('splash.error.title'));

// 事件
const emit = defineEmits<{
  complete: [];
}>();

// 过渡完成回调
const onTransitionComplete = () => {
  // 清理工作
  stopParticles();
};

// 键盘事件处理
const handleKeydown = (e: KeyboardEvent) => {
  if (!hasError.value) return;

  switch (e.key) {
    case 'r':
    case 'R':
      e.preventDefault();
      retryInitialization();
      break;
    case 'Escape':
      e.preventDefault();
      skipInitialization();
      break;
  }
};

// 生命周期
onMounted(async () => {
  // 获取应用版本
  try {
    version.value = await getVersion();
  } catch {
    version.value = '0.1.0';
  }

  // 启动粒子动画
  startParticles();

  // 触发动画
  setTimeout(() => {
    isReady.value = true;
  }, 100);

  // 开始初始化流程
  setTimeout(() => {
    runInitialization();
  }, 600);

  // 注册键盘事件
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  // 清理
  stopParticles();
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style lang="scss" scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.95) 0%, rgba(118, 75, 162, 0.95) 100%);
  background-size: 400% 400%;
  animation: gradientShift 15s ease infinite;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  overflow: hidden;
  border-radius: 12px;
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

/* 粒子动画背景 */
.particle-canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
}

/* 渐变覆盖层 - 浮动光效 */
.gradient-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
  z-index: 1;
  pointer-events: none;
  animation: float 6s ease-in-out infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px) scale(1);
  }
  50% {
    transform: translateY(-20px) scale(1.05);
  }
}

/* 中央内容 */
.splash-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  z-index: 2;
  padding: 40px;
  max-width: 480px;
  width: 90%;
  opacity: 0;
  transform: translateY(30px);
  transition: all 0.8s cubic-bezier(0.34, 1.56, 0.64, 1);

  &.content-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 标题 */
.splash-title {
  font-size: 26px;
  font-weight: 700;
  color: white;
  letter-spacing: 1px;
  margin: 0;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  opacity: 0;
  transform: translateY(15px);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.15s;

  &.title-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 版本号 */
.splash-version {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.9);
  font-weight: 500;
  letter-spacing: 1px;
  padding: 6px 14px;
  background: rgba(255, 255, 255, 0.15);
  border-radius: 20px;
  backdrop-filter: blur(10px);
  opacity: 0;
  transform: translateY(10px);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.25s;

  &.version-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 步骤容器 */
.steps-wrapper {
  width: 100%;
  opacity: 0;
  transform: translateY(20px) scale(0.95);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.35s;

  &.steps-ready {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

/* 键盘快捷键提示 */
.keyboard-hints {
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 20px;
  z-index: 101;

  .hint {
    font-size: 12px;
    color: rgba(255, 255, 255, 0.7);
    padding: 4px 10px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }
}

/* 离开过渡动画 */
.splash-fade-leave-active {
  transition: all 0.6s cubic-bezier(0.4, 0, 0.2, 1);
}

.splash-fade-leave-to {
  opacity: 0;
  transform: scale(1.05);
  filter: blur(10px);
}

/* 响应式设计 */
@media (max-width: 480px) {
  .splash-content {
    padding: 24px;
  }

  .splash-title {
    font-size: 22px;
  }

  .keyboard-hints {
    display: none;
  }
}
</style>
