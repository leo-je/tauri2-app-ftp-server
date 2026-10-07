<template>
  <el-dropdown trigger="click" placement="top-start" @command="handleLanguageChange">
    <button type="button" class="lang-row" :title="$t('language.title')">
      <SvgIcon name="global" :size="15" class="lang-globe" />
      <span class="lang-current">{{ currentLabel }}</span>
      <SvgIcon name="chevronUp" :size="14" class="lang-caret" />
    </button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item
          v-for="lang in languages"
          :key="lang.value"
          :command="lang.value"
          :class="{ 'is-active': currentLanguage === lang.value }"
        >
          <span class="lang-option">{{ lang.label }}</span>
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElDropdown, ElDropdownMenu, ElDropdownItem } from 'element-plus'
import { SvgIcon } from './icons'
import { invoke } from '@tauri-apps/api/core'
import appStore from '../store'
import { applyDocumentLang } from '../i18n'

const { locale, t } = useI18n()

/** 语言名一律用该语言自己的写法，所以与当前 locale 无关 */
const languages = computed(() => [
  { value: 'zh', label: t('language.zh') },
  { value: 'en', label: t('language.en') },
  { value: 'ja', label: t('language.ja') },
  { value: 'th', label: t('language.th') }
])

const currentLanguage = computed(() => locale.value)

const currentLabel = computed(
  () => languages.value.find((item) => item.value === currentLanguage.value)?.label ?? ''
)

const updateTrayMenu = async () => {
  try {
    await invoke('update_tray_menu_language', {
      isRunning: appStore.runtimeState.isServerRunning.value,
      text: {
        show: t('tray.show'),
        start: t('tray.start'),
        stop: t('tray.stop'),
        quit: t('tray.quit'),
        uptime: t('tray.uptime')
      }
    })
  } catch (e) {
    console.warn('Failed to update tray menu:', e)
  }
}

const handleLanguageChange = async (lang: string) => {
  locale.value = lang
  applyDocumentLang(lang)
  localStorage.setItem('app-language', lang)
  await updateTrayMenu()
}
</script>

<style scoped lang="scss">
.lang-row {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  width: 100%;
  height: 30px;
  padding: 0 var(--s-2);
  border: 1px solid transparent;
  border-radius: var(--r-md);
  background: transparent;
  color: var(--text-2);
  cursor: pointer;
  transition: background var(--dur-1) var(--ease-standard),
    color var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--surface-3);
    color: var(--text);
  }
}

.lang-globe {
  flex-shrink: 0;
  color: var(--text-3);
}

.lang-current {
  flex: 1;
  text-align: left;
  font-size: var(--fs-md);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.lang-caret {
  flex-shrink: 0;
  color: var(--text-3);
  opacity: 0;
  transition: opacity var(--dur-1) var(--ease-standard);
}

.lang-row:hover .lang-caret {
  opacity: 1;
}

.lang-option {
  display: block;
  min-width: 96px;
}
</style>
