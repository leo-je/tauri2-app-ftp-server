<template>
  <svg
    :viewBox="viewBox"
    :class="iconClass"
    :style="iconStyle"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      v-for="(path, index) in paths"
      :key="index"
      :d="path"
      v-bind="pathAttrs"
    />
  </svg>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { svgIcons, type IconName } from './icons';

interface Props {
  /** 图标名称 */
  name: IconName;
  /** 图标尺寸 */
  size?: number | string;
  /** 自定义类名 */
  class?: string;
  /** 是否填充模式 */
  fill?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  size: 24,
  class: '',
  fill: false
});

const iconConfig = computed(() => svgIcons[props.name]);

const paths = computed(() => iconConfig.value.paths);

const pathAttrs = computed(() => {
  const attrs: Record<string, string | number> = { ...iconConfig.value.attrs };

  // 处理 stroke 和 fill 的转换
  if (props.fill && attrs.fill === 'none') {
    attrs.fill = 'currentColor';
    delete attrs.stroke;
    delete attrs.strokeWidth;
    delete attrs.strokeLinecap;
    delete attrs.strokeLinejoin;
  }

  return attrs;
});

const viewBox = computed(() => '0 0 24 24');

const iconClass = computed(() => {
  return ['svg-icon', props.class].filter(Boolean).join(' ');
});

const iconStyle = computed(() => {
  const size = typeof props.size === 'number' ? `${props.size}px` : props.size;
  return {
    width: size,
    height: size
  };
});
</script>

<style scoped>
.svg-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  vertical-align: middle;
}
</style>
