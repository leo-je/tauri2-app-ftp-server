<template>
  <img
    :src="logoSrc"
    :class="['app-logo-img', props.class]"
    :style="logoStyle"
    alt="FTP Server"
    @error="handleError"
  />
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';

interface Props {
  /** 图标尺寸 */
  size?: number | string;
  /** 自定义类名 */
  class?: string;
}

const props = withDefaults(defineProps<Props>(), {
  size: 64,
  class: ''
});

// 使用 public 目录下的图标（打包后也能正常访问）
const logoSrc = ref('/icon.png');

// 备用图标
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

const logoStyle = computed(() => {
  const size = typeof props.size === 'number' ? `${props.size}px` : props.size;
  return {
    width: size,
    height: size,
    objectFit: 'contain' as const
  };
});
</script>

<style scoped>
.app-logo-img {
  display: inline-block;
  flex-shrink: 0;
  vertical-align: middle;
}
</style>
