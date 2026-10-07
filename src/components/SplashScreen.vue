<template>
  <Transition name="splash">
    <div v-if="visible" class="splash-screen">
      <AppBackground />

      <div class="splash-content" :class="{ 'is-ready': isReady }">
        <div class="logo-tile">
          <AppLogo :size="52" />
        </div>

        <div class="identity">
          <h1 class="splash-title">{{ $t('app.name') }}</h1>
          <span class="splash-version mono">v{{ version }}</span>
        </div>

        <ol class="steps">
          <li
            v-for="(step, index) in initSteps"
            :key="step.id"
            class="step"
            :class="`is-${step.status}`"
          >
            <span class="step-indicator" :title="$t(`splash.status.${step.status}`)">
              <SvgIcon v-if="step.status === 'completed'" name="check" :size="11" />
              <SvgIcon v-else-if="step.status === 'error'" name="close" :size="11" />
            </span>

            <span class="step-text">
              <span class="step-name">{{ step.name }}</span>
              <span class="step-detail">
                {{ step.message || step.description }}
              </span>
            </span>

            <span v-if="step.status === 'completed'" class="step-index mono">
              {{ String(index + 1).padStart(2, '0') }}
            </span>
          </li>
        </ol>

        <div class="progress">
          <div class="progress-track">
            <div
              class="progress-fill"
              :class="{ 'is-error': hasError }"
              :style="{ width: `${overallProgress}%` }"
            ></div>
          </div>
          <span class="progress-value mono">{{ overallProgress }}%</span>
        </div>

        <div v-if="hasError" class="error-panel">
          <div class="error-head">
            <SvgIcon name="info" :size="14" class="error-icon" />
            <span class="error-title">{{ errorTitle }}</span>
          </div>
          <p class="error-message">{{ errorMessage }}</p>
          <div class="error-actions">
            <button type="button" class="error-btn is-primary" @click="retryInitialization">
              {{ $t('splash.error.retry') }}
              <kbd>R</kbd>
            </button>
            <button type="button" class="error-btn" @click="skipInitialization">
              {{ $t('splash.error.skip') }}
              <kbd>ESC</kbd>
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { getVersion } from '@tauri-apps/api/app';
import AppLogo from './AppLogo.vue';
import AppBackground from './AppBackground.vue';
import { SvgIcon } from './icons';
import { useInitialization } from './splash/composables/useInitialization';

const REVEAL_DELAY = 100; // 入场动画起始延迟(ms)
const INIT_DELAY = 600; // 开始跑初始化之前的等待(ms)
const CLOSE_DELAY = 800; // 全部完成后停留多久再淡出(ms)

const { t } = useI18n();

const visible = ref(true);
const isReady = ref(false);
const version = ref('—');

const {
  initSteps,
  hasError,
  errorMessage,
  overallProgress,
  runInitialization,
  retryInitialization,
  skipInitialization
} = useInitialization({
  t,
  onComplete: () => {
    setTimeout(() => {
      emit('complete');
      visible.value = false;
    }, CLOSE_DELAY);
  },
  onError: (message) => {
    console.error('Initialization error:', message);
  }
});

const errorTitle = computed(() => t('splash.error.title'));

const emit = defineEmits<{
  complete: [];
}>();

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

onMounted(async () => {
  try {
    version.value = await getVersion();
  } catch {
    version.value = '—';
  }

  setTimeout(() => {
    isReady.value = true;
  }, REVEAL_DELAY);

  setTimeout(() => {
    runInitialization();
  }, INIT_DELAY);

  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped lang="scss">
.splash-screen {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  background: var(--bg);
  border-radius: 12px;
}

.splash-content {
  position: relative;
  z-index: 1;
  display: flex;
  flex-direction: column;
  width: 100%;
  max-width: 380px;
  padding: var(--s-6);
  opacity: 0;
  transform: translateY(10px);
  transition: opacity var(--dur-3) var(--ease-out), transform var(--dur-3) var(--ease-out);

  &.is-ready {
    opacity: 1;
    transform: translateY(0);
  }
}

/* ── 标识 ──────────────────────────────────────────────────── */
.logo-tile {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 72px;
  height: 72px;
  margin: 0 auto var(--s-4);
  border-radius: var(--r-lg);
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: var(--shadow-2);
}

.identity {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 3px;
  margin-bottom: var(--s-6);
}

.splash-title {
  font-size: var(--fs-lg);
  font-weight: 600;
  letter-spacing: -0.01em;
  color: var(--text);
}

.splash-version {
  font-size: var(--fs-xs);
  color: var(--text-3);
}

/* ── 初始化步骤 ────────────────────────────────────────────── */
.steps {
  display: flex;
  flex-direction: column;
  gap: 2px;
  list-style: none;
}

.step {
  display: flex;
  align-items: flex-start;
  gap: var(--s-3);
  padding: var(--s-2) var(--s-3);
  border-radius: var(--r-md);
  transition: background var(--dur-2) var(--ease-standard);

  &.is-running {
    background: var(--surface-2);
  }
}

/* 状态标记：空心点 / 转圈 / 对勾 / 叉 */
.step-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  margin-top: 1px;
  flex-shrink: 0;
  border-radius: var(--r-full);
  border: 1px solid var(--border-2);
  color: var(--text-inverse);
  transition: background var(--dur-2) var(--ease-standard),
    border-color var(--dur-2) var(--ease-standard);

  .step.is-completed & {
    background: var(--success);
    border-color: var(--success);
  }

  .step.is-error & {
    background: var(--danger);
    border-color: var(--danger);
  }

  .step.is-running & {
    border: 2px solid var(--accent-line);
    border-top-color: var(--accent);
    animation: spin 0.7s linear infinite;
  }
}

.step-text {
  display: flex;
  flex-direction: column;
  gap: 1px;
  flex: 1;
  min-width: 0;
}

.step-name {
  font-size: var(--fs-md);
  font-weight: 500;
  color: var(--text-3);
  transition: color var(--dur-2) var(--ease-standard);

  .step.is-running &,
  .step.is-completed & {
    color: var(--text);
  }

  .step.is-error & {
    color: var(--danger);
  }
}

.step-detail {
  font-size: var(--fs-sm);
  color: var(--text-3);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.step-index {
  flex-shrink: 0;
  font-size: var(--fs-2xs);
  color: var(--text-3);
  opacity: 0.6;
}

/* ── 进度 ──────────────────────────────────────────────────── */
.progress {
  display: flex;
  align-items: center;
  gap: var(--s-3);
  margin-top: var(--s-5);
}

.progress-track {
  flex: 1;
  height: 2px;
  border-radius: var(--r-full);
  background: var(--border);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: var(--r-full);
  background: var(--accent);
  transition: width var(--dur-3) var(--ease-out), background var(--dur-2) var(--ease-standard);

  &.is-error {
    background: var(--danger);
  }
}

.progress-value {
  flex-shrink: 0;
  min-width: 34px;
  text-align: right;
  font-size: var(--fs-xs);
  color: var(--text-3);
}

/* ── 错误 ──────────────────────────────────────────────────── */
.error-panel {
  margin-top: var(--s-4);
  padding: var(--s-3) var(--s-4);
  border: 1px solid var(--danger-line);
  border-radius: var(--r-md);
  background: var(--danger-soft);
}

.error-head {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--danger);
}

.error-icon {
  flex-shrink: 0;
}

.error-title {
  font-size: var(--fs-md);
  font-weight: 600;
}

.error-message {
  margin: var(--s-1) 0 var(--s-3);
  font-size: var(--fs-sm);
  line-height: 1.5;
  color: var(--text-2);
  word-break: break-word;
}

.error-actions {
  display: flex;
  gap: var(--s-2);
}

.error-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 28px;
  padding: 0 10px;
  border: 1px solid var(--border-2);
  border-radius: var(--r-sm);
  background: var(--surface);
  color: var(--text);
  font-size: var(--fs-sm);
  font-weight: 500;
  cursor: pointer;
  transition: background var(--dur-1) var(--ease-standard),
    border-color var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--surface-3);
    border-color: var(--border-strong);
  }

  &.is-primary {
    background: var(--danger);
    border-color: var(--danger);
    color: #fff;

    &:hover {
      filter: brightness(1.08);
    }
  }

  kbd {
    font-family: var(--font-mono);
    font-size: var(--fs-2xs);
    padding: 1px 4px;
    border-radius: var(--r-xs);
    background: rgba(255, 255, 255, 0.18);
    color: inherit;
  }

  &:not(.is-primary) kbd {
    background: var(--surface-3);
    color: var(--text-3);
  }
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* ── 淡出 ──────────────────────────────────────────────────── */
.splash-leave-active {
  transition: opacity var(--dur-3) var(--ease-standard);
}

.splash-leave-to {
  opacity: 0;
}
</style>
