<template>
  <div class="app-background" aria-hidden="true">
    <div class="grid-layer"></div>
    <div class="glow glow-a"></div>
    <div class="glow glow-b"></div>
    <div class="vignette-layer"></div>
  </div>
</template>

<script setup lang="ts">
/**
 * 背景只做三件事：给出尺度感（网格）、给出光源方向（暖色辉光）、收住边缘（暗角）。
 * 所有颜色都取自令牌，所以明暗两套主题共用同一个结构。
 */
</script>

<style scoped lang="scss">
.app-background {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  z-index: 0;
  isolation: isolate;
}

.grid-layer,
.glow,
.vignette-layer {
  position: absolute;
  inset: 0;
}

/* 方格纸：给内容一个可测量的底 */
.grid-layer {
  background-image: linear-gradient(var(--grid-line) 1px, transparent 1px),
    linear-gradient(90deg, var(--grid-line) 1px, transparent 1px);
  background-size: 32px 32px;
  mask-image: radial-gradient(circle at 50% 30%, rgba(0, 0, 0, 0.9) 0%, transparent 78%);
  opacity: 0.9;
}

.glow {
  border-radius: 50%;
  filter: blur(60px);
}

.glow-a {
  inset: auto;
  top: -18%;
  right: -14%;
  width: 60%;
  height: 46%;
  background: radial-gradient(circle, var(--glow-a) 0%, transparent 70%);
  animation: drift 42s var(--ease-standard) infinite;
}

.glow-b {
  inset: auto;
  bottom: -22%;
  left: -16%;
  width: 58%;
  height: 48%;
  background: radial-gradient(circle, var(--glow-b) 0%, transparent 70%);
  animation: drift 54s var(--ease-standard) infinite reverse;
}

.vignette-layer {
  background: radial-gradient(circle at 50% 40%, transparent 55%, var(--vignette) 100%);
}

@keyframes drift {
  0%,
  100% {
    transform: translate3d(0, 0, 0) scale(1);
  }
  50% {
    transform: translate3d(-4%, 5%, 0) scale(1.06);
  }
}

@media (prefers-reduced-motion: reduce) {
  .glow {
    animation: none;
  }
}
</style>
