/**
 * 启动页面类型定义
 * @module splash/types
 */

/** 步骤状态 */
export type InitStepStatus = 'pending' | 'running' | 'completed' | 'error';

/** 步骤类型 */
export type InitStepType = 'system_check' | 'config_load' | 'service_init' | 'ready';

/**
 * 初始化步骤
 * @interface InitStep
 */
export interface InitStep {
  /** 步骤ID */
  id: InitStepType;
  /** 步骤名称 */
  name: string;
  /** 步骤描述 */
  description: string;
  /** 当前状态 */
  status: InitStepStatus;
  /** 进度百分比 (0-100) */
  progress: number;
  /** 当前消息 */
  message: string;
}

/**
 * 后端初始化响应
 * @interface InitStepResponse
 */
export interface InitStepResponse {
  /** 是否成功 */
  success: boolean;
  /** 当前步骤 */
  step: string;
  /** 进度百分比 (0-100) */
  progress: number;
  /** 消息内容 */
  message: string;
  /** 是否可以继续 */
  can_continue: boolean;
  /** 预计剩余时间（毫秒） */
  estimated_time_ms?: number;
}

/**
 * 步骤配置
 * @interface StepConfig
 */
export interface StepConfig {
  /** 步骤ID */
  id: InitStepType;
  /** 名称的国际化键 */
  nameKey: string;
  /** 描述的国际化键 */
  descKey: string;
  /** 预计耗时（毫秒） */
  estimatedTime: number;
}

/**
 * 初始化步骤配置常量
 * @constant INIT_STEPS_CONFIG
 */
export const INIT_STEPS_CONFIG: StepConfig[] = [
  {
    id: 'system_check',
    nameKey: 'splash.init.systemCheck',
    descKey: 'splash.init.systemCheckDesc',
    estimatedTime: 800
  },
  {
    id: 'config_load',
    nameKey: 'splash.init.configLoad',
    descKey: 'splash.init.configLoadDesc',
    estimatedTime: 500
  },
  {
    id: 'service_init',
    nameKey: 'splash.init.serviceInit',
    descKey: 'splash.init.serviceInitDesc',
    estimatedTime: 1200
  },
  {
    id: 'ready',
    nameKey: 'splash.init.ready',
    descKey: 'splash.init.readyDesc',
    estimatedTime: 300
  }
];

/**
 * 创建初始步骤数组
 * @param t - 国际化翻译函数
 * @returns 初始化步骤数组
 * @example
 * const steps = createInitSteps((key) => t(key));
 */
export function createInitSteps(t: (key: string) => string): InitStep[] {
  return INIT_STEPS_CONFIG.map(config => ({
    id: config.id,
    name: t(config.nameKey),
    description: t(config.descKey),
    status: 'pending',
    progress: 0,
    message: ''
  }));
}
