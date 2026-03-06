<template>
  <Transition name="splash-fade" @after-leave="onTransitionComplete">
    <div v-if="visible" class="splash-screen">
      <!-- 粒子动画背景 -->
      <canvas ref="particleCanvas" class="particle-canvas"></canvas>

      <!-- 渐变背景覆盖层 -->
      <div class="gradient-overlay"></div>

      <!-- 中央内容 -->
      <div class="splash-content" :class="{ 'content-ready': isReady }">
        <!-- Logo 容器 -->
        <div class="logo-wrapper" :class="{ 'logo-ready': isReady }">
          <div class="logo-glow"></div>
          <div class="logo-ring ring-1"></div>
          <div class="logo-ring ring-2"></div>
          <div class="logo-ring ring-3"></div>
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

        <!-- 初始化步骤卡片（毛玻璃效果） -->
        <div class="init-steps-container" :class="{ 'steps-ready': isReady }">
          <div class="steps-header">
            <span class="steps-title">{{ $t('splash.init.title') }}</span>
            <span class="progress-text">{{ Math.round(overallProgress) }}%</span>
          </div>

          <div class="steps-list">
            <div
              v-for="(step, index) in initSteps"
              :key="step.id"
              class="step-item"
              :class="{
                'step-active': currentStepIndex === index,
                'step-completed': step.status === 'completed',
                'step-error': step.status === 'error'
              }"
            >
              <div class="step-icon">
                <div v-if="step.status === 'completed'" class="icon-completed">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                    <path d="M20 6L9 17l-5-5" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </div>
                <div v-else-if="step.status === 'error'" class="icon-error">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                    <path d="M6 18L18 6M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </div>
                <div v-else-if="step.status === 'running'" class="icon-loading">
                  <div class="spinner"></div>
                </div>
                <div v-else class="icon-pending">
                  {{ index + 1 }}
                </div>
              </div>

              <div class="step-content">
                <div class="step-name">{{ step.name }}</div>
                <div class="step-desc">{{ step.description }}</div>
              </div>

              <div v-if="step.message" class="step-message">
                {{ step.message }}
              </div>
            </div>
          </div>

          <!-- 进度条 -->
          <div class="progress-track">
            <div class="progress-bar" :style="{ width: `${overallProgress}%` }"></div>
            <div class="progress-glow" :style="{ width: `${overallProgress}%` }"></div>
          </div>
        </div>

        <!-- 错误提示 -->
        <Transition name="error-slide">
          <div v-if="hasError" class="error-container">
            <div class="error-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <path d="M12 8v4M12 16h.01"/>
              </svg>
            </div>
            <div class="error-content">
              <div class="error-title">{{ $t('splash.error.title') }}</div>
              <div class="error-message">{{ errorMessage }}</div>
            </div>
            <div class="error-actions">
              <el-button type="primary" size="small" @click="retryInitialization">
                {{ $t('splash.error.retry') }}
              </el-button>
              <el-button size="small" @click="skipInitialization">
                {{ $t('splash.error.skip') }}
              </el-button>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import AppLogo from './AppLogo.vue';
import { getVersion } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';

const { t } = useI18n();
const visible = ref(true);
const version = ref('0.1.0');
const isReady = ref(false);
const hasError = ref(false);
const errorMessage = ref('');
const currentStepIndex = ref(-1);
const particleCanvas = ref<HTMLCanvasElement | null>(null);

// 初始化步骤定义
interface InitStep {
  id: string;
  name: string;
  description: string;
  status: 'pending' | 'running' | 'completed' | 'error';
  progress: number;
  message: string;
}

const initSteps = ref<InitStep[]>([
  {
    id: 'system_check',
    name: t('splash.init.systemCheck'),
    description: t('splash.init.systemCheckDesc'),
    status: 'pending',
    progress: 0,
    message: ''
  },
  {
    id: 'config_load',
    name: t('splash.init.configLoad'),
    description: t('splash.init.configLoadDesc'),
    status: 'pending',
    progress: 0,
    message: ''
  },
  {
    id: 'service_init',
    name: t('splash.init.serviceInit'),
    description: t('splash.init.serviceInitDesc'),
    status: 'pending',
    progress: 0,
    message: ''
  },
  {
    id: 'ready',
    name: t('splash.init.ready'),
    description: t('splash.init.readyDesc'),
    status: 'pending',
    progress: 0,
    message: ''
  }
]);

// 计算总体进度
const overallProgress = computed(() => {
  const totalProgress = initSteps.value.reduce((sum, step) => sum + step.progress, 0);
  return totalProgress / initSteps.value.length;
});

const emit = defineEmits<{
  complete: [];
}>();

// 粒子动画
class ParticleSystem {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private particles: Particle[] = [];
  private animationId: number = 0;
  private mouseX = 0;
  private mouseY = 0;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d');
    if (!ctx) throw new Error('Could not get 2D context');
    this.ctx = ctx;
    this.resize();
    this.initParticles();
    this.animate();
    window.addEventListener('resize', () => this.resize());
    canvas.addEventListener('mousemove', (e) => {
      const rect = canvas.getBoundingClientRect();
      this.mouseX = e.clientX - rect.left;
      this.mouseY = e.clientY - rect.top;
    });
  }

  resize() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  initParticles() {
    const particleCount = Math.min(80, Math.floor((this.canvas.width * this.canvas.height) / 15000));
    this.particles = [];

    for (let i = 0; i < particleCount; i++) {
      this.particles.push(new Particle(
        Math.random() * this.canvas.width,
        Math.random() * this.canvas.height,
        Math.random() * 2 + 1
      ));
    }
  }

  animate() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    // 更新和绘制粒子
    this.particles.forEach((particle, index) => {
      particle.update(this.canvas.width, this.canvas.height);
      particle.draw(this.ctx);

      // 绘制连接线
      for (let j = index + 1; j < this.particles.length; j++) {
        const other = this.particles[j];
        const dx = particle.x - other.x;
        const dy = particle.y - other.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < 120) {
          const opacity = (1 - distance / 120) * 0.25;
          this.ctx.beginPath();
          this.ctx.strokeStyle = `rgba(102, 126, 234, ${opacity})`;
          this.ctx.lineWidth = 1;
          this.ctx.moveTo(particle.x, particle.y);
          this.ctx.lineTo(other.x, other.y);
          this.ctx.stroke();
        }
      }

      // 鼠标交互
      const dx = this.mouseX - particle.x;
      const dy = this.mouseY - particle.y;
      const distance = Math.sqrt(dx * dx + dy * dy);
      if (distance < 150) {
        const force = (150 - distance) / 150;
        particle.vx += (dx / distance) * force * 0.02;
        particle.vy += (dy / distance) * force * 0.02;
      }
    });

    this.animationId = requestAnimationFrame(() => this.animate());
  }

  destroy() {
    cancelAnimationFrame(this.animationId);
    window.removeEventListener('resize', () => this.resize());
  }
}

class Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  color: string;

  constructor(x: number, y: number, radius: number) {
    this.x = x;
    this.y = y;
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = (Math.random() - 0.5) * 0.5;
    this.radius = radius;

    const colors = ['#667eea', '#764ba2', '#f093fb', '#4facfe', '#00f2fe'];
    this.color = colors[Math.floor(Math.random() * colors.length)];
  }

  update(canvasWidth: number, canvasHeight: number) {
    this.x += this.vx;
    this.y += this.vy;

    // 边界反弹
    if (this.x < 0 || this.x > canvasWidth) this.vx *= -1;
    if (this.y < 0 || this.y > canvasHeight) this.vy *= -1;

    // 限制速度
    const maxSpeed = 1;
    const speed = Math.sqrt(this.vx * this.vx + this.vy * this.vy);
    if (speed > maxSpeed) {
      this.vx = (this.vx / speed) * maxSpeed;
      this.vy = (this.vy / speed) * maxSpeed;
    }
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
    ctx.fillStyle = this.color;
    ctx.fill();

    // 发光效果
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.radius * 3, 0, Math.PI * 2);
    const gradient = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, this.radius * 3);
    gradient.addColorStop(0, this.color + '40');
    gradient.addColorStop(1, 'transparent');
    ctx.fillStyle = gradient;
    ctx.fill();
  }
}

let particleSystem: ParticleSystem | null = null;

// 执行初始化步骤
async function runInitialization() {
  hasError.value = false;
  errorMessage.value = '';

  for (let i = 0; i < initSteps.value.length; i++) {
    currentStepIndex.value = i;
    const step = initSteps.value[i];

    // 重置后续步骤状态
    for (let j = i + 1; j < initSteps.value.length; j++) {
      initSteps.value[j].status = 'pending';
      initSteps.value[j].progress = 0;
      initSteps.value[j].message = '';
    }

    try {
      step.status = 'running';
      step.message = getStepMessage(step.id);

      // 调用后端初始化命令
      const result = await invoke<{
        success: boolean;
        step: string;
        progress: number;
        message: string;
        can_continue: boolean;
      }>('run_init_step', { step: step.id });

      if (result.success) {
        step.status = 'completed';
        step.progress = result.progress;
        step.message = result.message;
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      step.status = 'error';
      step.progress = 0;
      step.message = error instanceof Error ? error.message : t('splash.error.unknownError');
      hasError.value = true;
      errorMessage.value = step.message;
      return;
    }
  }

  // 所有步骤完成
  if (!hasError.value) {
    setTimeout(() => {
      emit('complete');
      visible.value = false;
    }, 800);
  }
}

// 获取步骤消息
function getStepMessage(stepId: string): string {
  switch (stepId) {
    case 'system_check':
      return t('splash.messages.checkingSystem');
    case 'config_load':
      return t('splash.messages.loadingConfig');
    case 'service_init':
      return t('splash.messages.initService');
    case 'ready':
      return t('splash.messages.finalizing');
    default:
      return '';
  }
}

// 重试初始化
async function retryInitialization() {
  // 重置所有步骤
  initSteps.value.forEach(step => {
    step.status = 'pending';
    step.progress = 0;
    step.message = '';
  });
  currentStepIndex.value = -1;
  hasError.value = false;
  errorMessage.value = '';

  // 重新开始初始化
  await runInitialization();
}

// 跳过初始化
function skipInitialization() {
  emit('complete');
  visible.value = false;
}

const onTransitionComplete = () => {
  // 过渡动画完成后的回调
};

onMounted(async () => {
  // 获取应用版本
  try {
    version.value = await getVersion();
  } catch {
    version.value = '0.1.0';
  }

  // 初始化粒子动画
  if (particleCanvas.value) {
    particleSystem = new ParticleSystem(particleCanvas.value);
  }

  // 触发动画
  setTimeout(() => {
    isReady.value = true;
  }, 100);

  // 开始初始化流程
  setTimeout(() => {
    runInitialization();
  }, 600);
});

onUnmounted(() => {
  if (particleSystem) {
    particleSystem.destroy();
  }
});
</script>

<style lang="scss" scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f0c29 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  overflow: hidden;
  border-radius: 12px;
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

/* 渐变覆盖层 */
.gradient-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background:
    radial-gradient(ellipse at 20% 20%, rgba(102, 126, 234, 0.15) 0%, transparent 50%),
    radial-gradient(ellipse at 80% 80%, rgba(118, 75, 162, 0.15) 0%, transparent 50%),
    radial-gradient(ellipse at 50% 50%, rgba(240, 147, 251, 0.05) 0%, transparent 70%);
  z-index: 1;
  pointer-events: none;
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

/* Logo 容器 */
.logo-wrapper {
  position: relative;
  width: 100px;
  height: 100px;
  opacity: 0;
  transform: scale(0.5);
  transition: all 0.8s cubic-bezier(0.34, 1.56, 0.64, 1);

  &.logo-ready {
    opacity: 1;
    transform: scale(1);
  }
}

.logo-glow {
  position: absolute;
  inset: -20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
  border-radius: 50%;
  filter: blur(30px);
  opacity: 0.4;
  animation: glowPulse 3s ease-in-out infinite;
}

.logo-ring {
  position: absolute;
  inset: -10px;
  border: 2px solid transparent;
  border-radius: 50%;
  border-top-color: rgba(102, 126, 234, 0.5);
  animation: ringRotate 3s linear infinite;

  &.ring-1 {
    animation-duration: 3s;
    border-top-color: rgba(102, 126, 234, 0.5);
  }

  &.ring-2 {
    inset: -15px;
    animation-duration: 4s;
    animation-direction: reverse;
    border-right-color: rgba(118, 75, 162, 0.3);
  }

  &.ring-3 {
    inset: -20px;
    animation-duration: 5s;
    border-bottom-color: rgba(240, 147, 251, 0.2);
  }
}

.splash-logo {
  position: relative;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, rgba(255, 255, 255, 0.05) 100%);
  border-radius: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  padding: 16px;
  overflow: hidden;

  :deep(.app-logo-img) {
    width: 100%;
    height: 100%;
    object-fit: contain;
    border-radius: 12px;
    animation: logoBreathe 4s ease-in-out infinite;
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
  color: rgba(255, 255, 255, 0.6);
  font-weight: 500;
  letter-spacing: 1px;
  padding: 6px 14px;
  background: rgba(255, 255, 255, 0.1);
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

/* 初始化步骤容器（毛玻璃效果） */
.init-steps-container {
  width: 100%;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 20px;
  margin-top: 10px;
  opacity: 0;
  transform: translateY(20px) scale(0.95);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.35s;

  &.steps-ready {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.steps-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.steps-title {
  font-size: 14px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.9);
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
  background: rgba(0, 0, 0, 0.2);
  transition: all 0.3s ease;
  border: 1px solid transparent;

  &.step-active {
    background: rgba(102, 126, 234, 0.15);
    border-color: rgba(102, 126, 234, 0.3);
  }

  &.step-completed {
    background: rgba(72, 187, 120, 0.1);
    border-color: rgba(72, 187, 120, 0.2);
  }

  &.step-error {
    background: rgba(245, 101, 101, 0.15);
    border-color: rgba(245, 101, 101, 0.3);
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

  .icon-pending {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.5);
    border-radius: 50%;
  }

  .icon-loading {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(102, 126, 234, 0.3);
    border-radius: 50%;

    .spinner {
      width: 14px;
      height: 14px;
      border: 2px solid transparent;
      border-top-color: #667eea;
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
    background: rgba(72, 187, 120, 0.3);
    color: #48bb78;
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
    background: rgba(245, 101, 101, 0.3);
    color: #f56565;
    border-radius: 50%;

    svg {
      width: 14px;
      height: 14px;
    }
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
  color: rgba(255, 255, 255, 0.5);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.step-message {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.6);
  margin-top: 4px;
  font-style: italic;
}

/* 进度条 */
.progress-track {
  position: relative;
  height: 4px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
  overflow: hidden;
  margin-top: 16px;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
  border-radius: 2px;
  transition: width 0.3s ease;
  position: relative;

  &::after {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 20px;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5));
    animation: progressShine 1.5s ease-in-out infinite;
  }
}

.progress-glow {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(102, 126, 234, 0.4), transparent);
  border-radius: 2px;
  filter: blur(4px);
  transition: width 0.3s ease;
}

/* 错误提示 */
.error-container {
  position: fixed;
  bottom: 40px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background: rgba(20, 20, 30, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 12px;
  border: 1px solid rgba(245, 101, 101, 0.3);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
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
  background: rgba(245, 101, 101, 0.2);
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
  color: rgba(255, 255, 255, 0.95);
  margin-bottom: 4px;
}

.error-message {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.error-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

/* 动画定义 */
@keyframes glowPulse {
  0%, 100% {
    opacity: 0.4;
    transform: scale(1);
  }
  50% {
    opacity: 0.6;
    transform: scale(1.1);
  }
}

@keyframes ringRotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
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

@keyframes spinnerRotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
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

/* 错误提示动画 */
.error-slide-enter-active,
.error-slide-leave-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.error-slide-enter-from,
.error-slide-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
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

  .init-steps-container {
    padding: 16px;
  }

  .step-desc {
    display: none;
  }

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
