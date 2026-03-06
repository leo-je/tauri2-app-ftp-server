<template>
  <div class="progress-track">
    <div
      class="progress-bar"
      :style="{ width: `${clampedProgress}%` }"
    >
      <div v-if="showShine" class="progress-shine"></div>
    </div>
    <div
      v-if="showGlow"
      class="progress-glow"
      :style="{ width: `${clampedProgress}%` }"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  progress: number;
  showGlow?: boolean;
  showShine?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showGlow: true,
  showShine: true
});

const clampedProgress = computed(() => {
  return Math.max(0, Math.min(100, props.progress));
});
</script>

<style lang="scss" scoped>
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
  overflow: hidden;
}

.progress-shine {
  position: absolute;
  top: 0;
  right: 0;
  width: 20px;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5));
  animation: progressShine 1.5s ease-in-out infinite;
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
  pointer-events: none;
}

@keyframes progressShine {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}
</style>
