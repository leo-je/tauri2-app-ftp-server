<template>
  <div
    class="logo-wrapper"
    :class="{ 'logo-ready': ready }"
    :style="wrapperStyle"
  >
    <!-- 发光效果 -->
    <div class="logo-glow"></div>

    <!-- 旋转环 -->
    <div class="logo-ring ring-1"></div>
    <div class="logo-ring ring-2"></div>
    <div class="logo-ring ring-3"></div>

    <!-- Logo 容器 -->
    <div class="splash-logo" :style="logoContainerStyle">
      <img
        :src="logoSrc"
        :class="['app-logo-img', { 'breathing': ready }]"
        :style="imgStyle"
        alt="FTP Server"
        @error="handleError"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

interface Props {
  size?: number | string;
  ready?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  size: 72,
  ready: false
});

const logoSrc = ref('/icon.png');
const fallbackIcons = [
  '/vite.svg',
  '/tauri.svg'
];
let fallbackIndex = 0;

const handleError = () => {
  if (fallbackIndex < fallbackIcons.length) {
    logoSrc.value = fallbackIcons[fallbackIndex];
    fallbackIndex++;
  }
};

const sizeValue = computed(() => {
  return typeof props.size === 'number' ? props.size : parseInt(props.size, 10);
});

const wrapperStyle = computed(() => ({
  width: `${sizeValue.value + 28}px`,
  height: `${sizeValue.value + 28}px`
}));

const logoContainerStyle = computed(() => ({
  width: `${sizeValue.value}px`,
  height: `${sizeValue.value}px`,
  borderRadius: `${sizeValue.value / 3}px`
}));

const imgStyle = computed(() => ({
  width: `${sizeValue.value - 16}px`,
  height: `${sizeValue.value - 16}px`
}));
</script>

<style lang="scss" scoped>
.logo-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
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
  opacity: 0.6;
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
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.25) 0%, rgba(255, 255, 255, 0.1) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
  padding: 8px;
  overflow: hidden;
}

.app-logo-img {
  object-fit: contain;
  border-radius: 12px;

  &.breathing {
    animation: logoBreathe 4s ease-in-out infinite;
  }
}

@keyframes glowPulse {
  0%, 100% {
    opacity: 0.6;
    transform: scale(1);
  }
  50% {
    opacity: 0.8;
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
</style>
