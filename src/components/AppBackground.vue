<template>
  <div
    class="app-background"
    :class="{ 'reduced-motion': isReducedMotion }"
    aria-hidden="true"
  >
    <canvas
      v-if="!isReducedMotion"
      ref="particleCanvas"
      class="particle-layer"
    ></canvas>

    <div class="aurora-layer">
      <span class="aurora aurora-a"></span>
      <span class="aurora aurora-b"></span>
      <span class="aurora aurora-c"></span>
    </div>

    <svg
      class="line-layer"
      viewBox="0 0 1440 900"
      preserveAspectRatio="none"
    >
      <path
        class="stream stream-a"
        d="M-120 140 C 120 280, 340 20, 560 160 S 970 360, 1240 180 S 1560 60, 1680 210"
      />
      <path
        class="stream stream-b"
        d="M-80 540 C 180 430, 360 680, 620 560 S 980 360, 1240 480 S 1560 780, 1680 620"
      />
      <path
        class="stream stream-c"
        d="M160 -120 C 260 120, 520 240, 720 220 S 1020 200, 1160 420 S 1120 840, 1380 960"
      />
      <path
        class="stream stream-d"
        d="M20 980 C 220 760, 340 560, 520 500 S 860 520, 1030 360 S 1230 120, 1510 -80"
      />
    </svg>

    <div class="grid-layer"></div>
    <div class="beam-layer">
      <span class="beam beam-a"></span>
      <span class="beam beam-b"></span>
      <span class="beam beam-c"></span>
    </div>
    <div class="vignette-layer"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useParticleAnimation } from './splash/composables/useParticleAnimation';

const particleCanvas = ref<HTMLCanvasElement | null>(null);

const { isReducedMotion, start } = useParticleAnimation(particleCanvas, {
  maxParticles: 52,
  connectionDistance: 160,
  enableMouseInteraction: false
});

onMounted(() => {
  requestAnimationFrame(() => {
    start();
  });
});
</script>

<style lang="scss" scoped>
.app-background {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  z-index: 0;
  isolation: isolate;
}

.particle-layer,
.line-layer,
.grid-layer,
.beam-layer,
.aurora-layer,
.vignette-layer {
  position: absolute;
  inset: 0;
}

.particle-layer {
  width: 100%;
  height: 100%;
  opacity: 0.7;
  mix-blend-mode: screen;
}

.aurora-layer {
  inset: -18%;
  filter: blur(28px) saturate(120%);
  opacity: 0.85;
}

.aurora {
  position: absolute;
  border-radius: 50%;
  mix-blend-mode: screen;
}

.aurora-a {
  top: -6%;
  left: -2%;
  width: 38vw;
  height: 38vw;
  background: radial-gradient(circle, var(--bg-aurora-primary) 0%, rgba(102, 126, 234, 0.08) 36%, transparent 72%);
  animation: auroraFloatA 24s ease-in-out infinite;
}

.aurora-b {
  top: 10%;
  right: -8%;
  width: 32vw;
  height: 32vw;
  background: radial-gradient(circle, var(--bg-aurora-secondary) 0%, rgba(240, 147, 251, 0.08) 38%, transparent 72%);
  animation: auroraFloatB 28s ease-in-out infinite;
}

.aurora-c {
  bottom: -14%;
  left: 28%;
  width: 44vw;
  height: 44vw;
  background: radial-gradient(circle, var(--bg-aurora-tertiary) 0%, rgba(79, 172, 254, 0.08) 34%, transparent 74%);
  animation: auroraFloatC 30s ease-in-out infinite;
}

.line-layer {
  width: 100%;
  height: 100%;
  opacity: 0.5;
  filter: drop-shadow(0 0 10px rgba(79, 172, 254, 0.18));
}

.stream {
  fill: none;
  stroke-width: 1.4;
  stroke-linecap: round;
  stroke-dasharray: 12 18 120 24;
  animation: dashFlow 18s linear infinite;
}

.stream-a,
.stream-d {
  stroke: rgba(255, 255, 255, 0.3);
}

.stream-b {
  stroke: rgba(93, 214, 255, 0.34);
  animation-duration: 22s;
}

.stream-c {
  stroke: rgba(244, 157, 255, 0.26);
  animation-duration: 26s;
  animation-direction: reverse;
}

.grid-layer {
  background-image:
    linear-gradient(var(--bg-grid-color) 1px, transparent 1px),
    linear-gradient(90deg, var(--bg-grid-color) 1px, transparent 1px);
  background-size: 144px 144px;
  mask-image: radial-gradient(circle at center, rgba(0, 0, 0, 0.88) 30%, transparent 88%);
  opacity: 0.4;
  animation: gridDrift 22s linear infinite;
}

.beam-layer {
  overflow: hidden;
}

.beam {
  position: absolute;
  height: 1px;
  border-radius: 999px;
  background: linear-gradient(90deg, transparent 0%, rgba(255, 255, 255, 0.9) 40%, rgba(100, 218, 255, 0.9) 60%, transparent 100%);
  box-shadow: 0 0 18px rgba(100, 218, 255, 0.38);
  opacity: 0.7;
}

.beam-a {
  top: 24%;
  left: -16%;
  width: 42%;
  transform: rotate(12deg);
  animation: beamSweepA 13s ease-in-out infinite;
}

.beam-b {
  top: 70%;
  right: -20%;
  width: 36%;
  transform: rotate(-10deg);
  animation: beamSweepB 16s ease-in-out infinite;
}

.beam-c {
  top: 48%;
  left: 42%;
  width: 24%;
  transform: rotate(-38deg);
  animation: beamSweepC 14s ease-in-out infinite;
}

.vignette-layer {
  background:
    radial-gradient(circle at center, transparent 0%, transparent 46%, rgba(3, 6, 18, 0.18) 100%),
    linear-gradient(180deg, rgba(255, 255, 255, 0.04) 0%, transparent 18%, rgba(1, 4, 14, 0.12) 100%);
}

.reduced-motion {
  .aurora,
  .stream,
  .grid-layer,
  .beam {
    animation: none;
  }
}

@keyframes dashFlow {
  from {
    stroke-dashoffset: 0;
  }
  to {
    stroke-dashoffset: -320;
  }
}

@keyframes gridDrift {
  from {
    transform: translate3d(0, 0, 0);
  }
  to {
    transform: translate3d(-72px, -48px, 0);
  }
}

@keyframes auroraFloatA {
  0%,
  100% {
    transform: translate3d(0, 0, 0) scale(1);
  }
  50% {
    transform: translate3d(10%, 8%, 0) scale(1.08);
  }
}

@keyframes auroraFloatB {
  0%,
  100% {
    transform: translate3d(0, 0, 0) scale(1);
  }
  50% {
    transform: translate3d(-12%, 10%, 0) scale(1.1);
  }
}

@keyframes auroraFloatC {
  0%,
  100% {
    transform: translate3d(0, 0, 0) scale(1);
  }
  50% {
    transform: translate3d(-10%, -10%, 0) scale(1.06);
  }
}

@keyframes beamSweepA {
  0%,
  100% {
    transform: translate3d(0, 0, 0) rotate(12deg);
    opacity: 0.12;
  }
  45% {
    opacity: 0.78;
  }
  50% {
    transform: translate3d(170%, 10px, 0) rotate(12deg);
    opacity: 0;
  }
}

@keyframes beamSweepB {
  0%,
  100% {
    transform: translate3d(0, 0, 0) rotate(-10deg);
    opacity: 0.08;
  }
  50% {
    transform: translate3d(-180%, -14px, 0) rotate(-10deg);
    opacity: 0.7;
  }
}

@keyframes beamSweepC {
  0%,
  100% {
    transform: translate3d(-24px, -24px, 0) rotate(-38deg);
    opacity: 0.1;
  }
  50% {
    transform: translate3d(120px, 80px, 0) rotate(-38deg);
    opacity: 0.62;
  }
}

@media (max-width: 960px) {
  .aurora-a,
  .aurora-b,
  .aurora-c {
    width: 62vw;
    height: 62vw;
  }

  .beam-c {
    display: none;
  }
}

@media (prefers-reduced-motion: reduce) {
  .aurora,
  .stream,
  .grid-layer,
  .beam {
    animation: none !important;
  }
}
</style>
