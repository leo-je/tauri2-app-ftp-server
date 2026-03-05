import { createI18n } from 'vue-i18n'
import zh from './locales/zh'
import en from './locales/en'

const messages = {
  zh,
  en
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
