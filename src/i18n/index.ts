import { createI18n } from 'vue-i18n'
import zh from './locales/zh'
import en from './locales/en'
import ja from './locales/ja'
import th from './locales/th'

const messages = {
  zh,
  en,
  ja,
  th
}

// 从 localStorage 读取语言设置，默认中文
const savedLanguage = localStorage.getItem('app-language') || 'zh'

const i18n = createI18n({
  legacy: false,
  locale: savedLanguage,
  fallbackLocale: 'zh',
  messages
})

export default i18n
