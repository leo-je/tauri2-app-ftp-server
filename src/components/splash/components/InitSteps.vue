<template>
  <div class="init-steps-container">
    <div class="steps-header">
      <span class="steps-title">{{ $t('splash.init.title') }}</span>
      <span class="progress-text">{{ Math.round(progress) }}%</span>
    </div>

    <div class="steps-list">
      <div
        v-for="(step, index) in steps"
        :key="step.id"
        class="step-item"
        :class="{
          'step-active': currentIndex === index,
          'step-completed': step.status === 'completed',
          'step-error': step.status === 'error'
        }"
      >
        <div class="step-icon">
          <!-- 已完成 -->
          <div v-if="step.status === 'completed'" class="icon-completed">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <path d="M20 6L9 17l-5-5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <!-- 错误 -->
          <div v-else-if="step.status === 'error'" class="icon-error">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <path d="M6 18L18 6M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <!-- 进行中 -->
          <div v-else-if="step.status === 'running'" class="icon-loading">
            <div class="spinner"></div>
          </div>
          <!-- 等待中 -->
          <div v-else class="icon-pending">
            {{ index + 1 }}
          </div>
        </div>

        <div class="step-content">
          <div class="step-name">{{ step.name }}</div>
          <div class="step-desc">{{ step.description }}</div>
          <div v-if="step.message" class="step-message">
            {{ step.message }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { InitStep } from '../types';

interface Props {
  steps: InitStep[];
  currentIndex: number;
  progress: number;
}

defineProps<Props>();
</script>

<style lang="scss" scoped>
.init-steps-container {
  width: 100%;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(10px);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 20px;
}

.steps-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
}

.steps-title {
  font-size: 14px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.95);
}

.progress-text {
  font-size: 14px;
  font-weight: 700;
  color: #667eea;
  font-variant-numeric: tabular-nums;
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.step-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
  border: 1px solid transparent;

  &.step-active {
    background: rgba(102, 126, 234, 0.25);
    border-color: rgba(102, 126, 234, 0.4);
  }

  &.step-completed {
    background: rgba(72, 187, 120, 0.2);
    border-color: rgba(72, 187, 120, 0.3);
  }

  &.step-error {
    background: rgba(245, 101, 101, 0.2);
    border-color: rgba(245, 101, 101, 0.4);
  }
}

.step-icon {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-size: 12px;
  font-weight: 700;
  transition: all 0.3s ease;
}

.icon-pending {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.2);
  color: rgba(255, 255, 255, 0.7);
  border-radius: 50%;
}

.icon-loading {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(102, 126, 234, 0.4);
  border-radius: 50%;

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid transparent;
    border-top-color: white;
    border-radius: 50%;
    animation: spinnerRotate 0.8s linear infinite;
  }
}

.icon-completed {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(72, 187, 120, 0.4);
  color: white;
  border-radius: 50%;

  svg {
    width: 14px;
    height: 14px;
  }
}

.icon-error {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(245, 101, 101, 0.4);
  color: white;
  border-radius: 50%;

  svg {
    width: 14px;
    height: 14px;
  }
}

.step-content {
  flex: 1;
  min-width: 0;
}

.step-name {
  font-size: 14px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.95);
  margin-bottom: 2px;
}

.step-desc {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.7);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.step-message {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.8);
  margin-top: 4px;
  font-style: italic;
}

@keyframes spinnerRotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

/* 响应式 */
@media (max-width: 480px) {
  .init-steps-container {
    padding: 16px;
  }

  .step-desc {
    display: none;
  }
}
</style>
