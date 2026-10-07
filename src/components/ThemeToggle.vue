<template>
  <div class="theme-toggle" role="radiogroup" :aria-label="$t('theme.title')">
    <button
      v-for="option in options"
      :key="option.value"
      type="button"
      class="theme-seg"
      :class="{ 'is-active': mode === option.value }"
      :title="option.label"
      :aria-label="option.label"
      :aria-pressed="mode === option.value"
      @click="setThemeMode(option.value)"
    >
      <SvgIcon :name="option.icon" :size="14" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { SvgIcon, type IconName } from './icons';
import { useTheme, setThemeMode, type ThemeMode } from '../composables/useTheme';

const { t } = useI18n();
const { mode } = useTheme();

const options = computed<Array<{ value: ThemeMode; icon: IconName; label: string }>>(() => [
  { value: 'light', icon: 'sun', label: t('theme.light') },
  { value: 'system', icon: 'monitor', label: t('theme.system') },
  { value: 'dark', icon: 'moon', label: t('theme.dark') }
]);
</script>

<style scoped lang="scss">
.theme-toggle {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 2px;
  padding: 2px;
  background: var(--chrome-inset);
  border: 1px solid var(--border);
  border-radius: var(--r-md);
}

.theme-seg {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 24px;
  border: 0;
  border-radius: var(--r-sm);
  background: transparent;
  color: var(--text-3);
  cursor: pointer;
  transition: background var(--dur-1) var(--ease-standard),
    color var(--dur-1) var(--ease-standard),
    box-shadow var(--dur-1) var(--ease-standard);

  &:hover {
    color: var(--text);
  }

  &.is-active {
    background: var(--chrome-raised);
    color: var(--accent);
    box-shadow: var(--shadow-1);
  }
}
</style>
