<template>
  <div class="language-switcher">
    <el-dropdown trigger="click" @command="handleLanguageChange">
      <div class="lang-btn" :title="$t('language.title')">
        <SvgIcon name="global" :size="18" />
      </div>
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
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElDropdown, ElDropdownMenu, ElDropdownItem } from 'element-plus'
import { SvgIcon } from './icons'

const { locale, t } = useI18n()

const languages = [
  { value: 'zh', label: t('language.zh') },
  { value: 'en', label: t('language.en') },
  { value: 'ja', label: t('language.ja') },
  { value: 'th', label: t('language.th') }
]

const currentLanguage = computed(() => locale.value)

const handleLanguageChange = (lang: string) => {
  locale.value = lang
  localStorage.setItem('app-language', lang)
}
</script>

<style lang="scss" scoped>
.language-switcher {
  display: flex;
  align-items: center;
}

.lang-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s ease;
  color: rgba(255, 255, 255, 0.95);
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.9) 0%, rgba(118, 75, 162, 0.9) 100%);
  border: 1px solid rgba(255, 255, 255, 0.3);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3), 0 2px 4px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);

  &:hover {
    background: linear-gradient(135deg, rgba(102, 126, 234, 1) 0%, rgba(118, 75, 162, 1) 100%);
    box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4), 0 2px 4px rgba(0, 0, 0, 0.15);
    transform: translateY(-1px) scale(1.05);
  }

  &:active {
    transform: translateY(0) scale(1);
    box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
  }

  svg {
    width: 18px;
    height: 18px;
  }
}

:deep(.el-dropdown-menu) {
  min-width: 120px;

  .el-dropdown-item {
    padding: 8px 16px;
    font-size: 13px;

    &.is-active {
      color: var(--el-color-primary);
      font-weight: 600;
      background: rgba(102, 126, 234, 0.1);
    }

    &:hover {
      background: rgba(102, 126, 234, 0.05);
    }
  }
}

.lang-option {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
