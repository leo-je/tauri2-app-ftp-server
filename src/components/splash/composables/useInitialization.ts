import { ref, computed, Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { InitStep, InitStepType, createInitSteps } from '../types';

export interface UseInitializationOptions {
  t: (key: string, params?: string[]) => string;
  onComplete?: () => void;
  onError?: (message: string) => void;
}

export interface UseInitializationReturn {
  initSteps: Ref<InitStep[]>;
  currentStepIndex: Ref<number>;
  hasError: Ref<boolean>;
  errorMessage: Ref<string>;
  overallProgress: Ref<number>;
  isRunning: Ref<boolean>;
  runInitialization: () => Promise<void>;
  retryInitialization: () => Promise<void>;
  skipInitialization: () => void;
  resetSteps: () => void;
}

export function useInitialization(options: UseInitializationOptions): UseInitializationReturn {
  const { t, onComplete, onError } = options;

  // 状态定义
  const initSteps = ref<InitStep[]>(createInitSteps(t));
  const currentStepIndex = ref(-1);
  const hasError = ref(false);
  const errorMessage = ref('');
  const isRunning = ref(false);

  // 计算总体进度
  const overallProgress = computed(() => {
    const total = initSteps.value.reduce((sum, step) => sum + step.progress, 0);
    return Math.round(total / initSteps.value.length);
  });

  // 执行单个步骤
  async function executeStep(index: number): Promise<boolean> {
    const step = initSteps.value[index];
    currentStepIndex.value = index;

    // 重置后续步骤
    for (let i = index + 1; i < initSteps.value.length; i++) {
      initSteps.value[i].status = 'pending';
      initSteps.value[i].progress = 0;
      initSteps.value[i].message = '';
    }

    try {
      step.status = 'running';
      step.message = getStepMessage(step.id);

      const result = await invoke<{
        success: boolean;
        step: string;
        progress: number;
        message: string;
        message_key: string;
        message_params: string[];
        can_continue: boolean;
      }>('run_init_step', { step: step.id });

      if (result.success) {
        step.status = 'completed';
        step.progress = result.progress;
        // 使用 message_key 进行翻译，如果有参数则传递参数
        if (result.message_key) {
          step.message = t(result.message_key, result.message_params);
        } else {
          step.message = result.message;
        }
        return true;
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      step.status = 'error';
      step.progress = 0;
      step.message = error instanceof Error ? error.message : t('splash.error.unknownError');
      hasError.value = true;
      errorMessage.value = step.message;
      onError?.(step.message);
      return false;
    }
  }

  // 获取步骤消息
  function getStepMessage(stepId: InitStepType): string {
    const messageMap: Record<InitStepType, string> = {
      system_check: t('splash.messages.checkingSystem'),
      config_load: t('splash.messages.loadingConfig'),
      service_init: t('splash.messages.initService'),
      ready: t('splash.messages.finalizing')
    };
    return messageMap[stepId];
  }

  // 执行完整初始化
  async function runInitialization(): Promise<void> {
    if (isRunning.value) return;

    isRunning.value = true;
    hasError.value = false;
    errorMessage.value = '';

    for (let i = 0; i < initSteps.value.length; i++) {
      const success = await executeStep(i);
      if (!success) {
        isRunning.value = false;
        return;
      }
    }

    isRunning.value = false;
    onComplete?.();
  }

  // 重试初始化
  async function retryInitialization(): Promise<void> {
    resetSteps();
    await runInitialization();
  }

  // 跳过初始化
  function skipInitialization(): void {
    isRunning.value = false;
    onComplete?.();
  }

  // 重置步骤
  function resetSteps(): void {
    initSteps.value = createInitSteps(t);
    currentStepIndex.value = -1;
    hasError.value = false;
    errorMessage.value = '';
  }

  return {
    initSteps,
    currentStepIndex,
    hasError,
    errorMessage,
    overallProgress,
    isRunning,
    runInitialization,
    retryInitialization,
    skipInitialization,
    resetSteps
  };
}
