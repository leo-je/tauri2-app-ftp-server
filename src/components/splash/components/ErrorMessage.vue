<template>
  <Transition name="error-slide">
    <div v-if="visible" class="error-container">
      <div class="error-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4M12 16h.01"/>
        </svg>
      </div>
      <div class="error-content">
        <div class="error-title">{{ title }}</div>
        <div class="error-message">{{ message }}</div>
      </div>
      <div class="error-actions">
        <el-button type="primary" size="small" @click="handleRetry">
          {{ $t('splash.error.retry') }}
        </el-button>
        <el-button size="small" @click="handleSkip">
          {{ $t('splash.error.skip') }}
        </el-button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ElButton } from 'element-plus';

interface Props {
  title: string;
  message: string;
  visible: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  retry: [];
  skip: [];
}>();

const handleRetry = () => {
  emit('retry');
};

const handleSkip = () => {
  emit('skip');
};
</script>

<style lang="scss" scoped>
.error-container {
  position: fixed;
  bottom: 40px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  border: 1px solid rgba(245, 101, 101, 0.25);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1), 0 4px 12px rgba(245, 101, 101, 0.1);
  z-index: 100;
  max-width: 420px;
  width: 90%;
}

.error-icon {
  width: 40px;
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(245, 101, 101, 0.15);
  border-radius: 50%;
  color: #f56565;

  svg {
    width: 24px;
    height: 24px;
  }
}

.error-content {
  flex: 1;
  min-width: 0;
}

.error-title {
  font-size: 14px;
  font-weight: 600;
  color: rgba(0, 0, 0, 0.85);
  margin-bottom: 4px;
}

.error-message {
  font-size: 12px;
  color: rgba(0, 0, 0, 0.5);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.error-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

/* 动画 */
.error-slide-enter-active,
.error-slide-leave-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.error-slide-enter-from,
.error-slide-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
}

/* 响应式 */
@media (max-width: 480px) {
  .error-container {
    flex-direction: column;
    text-align: center;
    padding: 20px;
  }

  .error-actions {
    width: 100%;
    justify-content: center;
  }
}
</style>
