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

// 使用 Tauri 的图标路径
const logoSrc = ref('/src-tauri/icons/icon.png');

// 如果主图标加载失败，尝试使用备用图标
const fallbackIcons = [
  '/src-tauri/icons/128x128.png',
  '/src-tauri/icons/64x64.png',
  '/src-tauri/icons/32x32.png'
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
