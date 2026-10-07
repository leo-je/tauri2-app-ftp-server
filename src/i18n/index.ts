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

/** 语言代码对应的 BCP-47 标签，供 <html lang> 使用 */
const HTML_LANG: Record<string, string> = {
  zh: 'zh-CN',
  en: 'en',
  ja: 'ja',
  th: 'th'
}

/**
 * 同步 <html lang>。
 * 只切 vue-i18n 的话标签页始终停在 zh-CN，屏幕阅读器会用中文音去读英文/日文内容。
 */
export function applyDocumentLang(locale: string) {
  document.documentElement.lang = HTML_LANG[locale] ?? locale
}

// 从 localStorage 读取语言设置，默认中文
const savedLanguage = localStorage.getItem('app-language') || 'zh'

const i18n = createI18n({
  legacy: false,
  locale: savedLanguage,
  fallbackLocale: 'zh',
  messages
})

applyDocumentLang(savedLanguage)

export default i18n
