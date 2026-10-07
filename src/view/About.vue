<template>
  <div class="about-page">
    <!-- ── 标识 ──────────────────────────────────────────────────── -->
    <section class="hero">
      <div class="logo-wrap">
        <AppLogo :size="56" />
      </div>
      <h1 class="app-name">{{ $t('about.appName') }}</h1>
      <p class="developer">
        <SvgIcon name="user" :size="13" class="developer-icon" />
        <span>{{ $t('about.developer') }} · {{ $t('about.developerName') }}</span>
      </p>
    </section>

    <!-- ── 系统信息 ──────────────────────────────────────────────── -->
    <section class="card">
      <div class="card-head">
        <SvgIcon name="monitor" :size="16" class="card-head-icon" />
        <span class="card-title">{{ $t('about.systemInfo') }}</span>
      </div>
      <div class="card-body info-list">
        <button
          type="button"
          class="info-row is-action"
          :title="$t('about.clickToCopy')"
          @click="copyVersion"
        >
          <span class="info-label">{{ $t('about.version') }}</span>
          <span class="info-value mono selectable">{{ version }}</span>
          <SvgIcon
            :name="versionCopied ? 'checkCircle' : 'copy'"
            :size="13"
            class="info-copy"
            :class="{ 'is-copied': versionCopied }"
          />
        </button>
        <div class="info-row">
          <span class="info-label">{{ $t('about.platform') }}</span>
          <span class="info-value mono">{{ platformName }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">{{ $t('about.arch') }}</span>
          <span class="info-value mono">{{ archName }}</span>
        </div>
      </div>
    </section>

    <!-- ── 技术栈 ────────────────────────────────────────────────── -->
    <section class="card">
      <div class="card-head">
        <SvgIcon name="terminal" :size="16" class="card-head-icon" />
        <span class="card-title">{{ $t('about.techStack') }}</span>
      </div>
      <div class="card-body tech-grid">
        <div v-for="tech in techStack" :key="tech.name" class="tech-item">
          <span class="tech-mark" :style="{ color: tech.color }">
            <SvgIcon :name="tech.icon" :size="17" />
          </span>
          <span class="tech-text">
            <span class="tech-name">{{ tech.name }}</span>
            <span class="tech-version mono">{{ tech.version }}</span>
          </span>
        </div>
      </div>
    </section>

    <footer class="copyright">
      <p>{{ $t('about.copyright') }}</p>
      <p>{{ $t('about.rights') }}</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { getVersion } from '@tauri-apps/api/app';
import { platform, arch } from '@tauri-apps/plugin-os';
import AppLogo from '../components/AppLogo.vue';
import { SvgIcon, type IconName } from '../components/icons';
import clipboard from 'tauri-plugin-clipboard-api';

const { t } = useI18n();
const version = ref('—');
const versionCopied = ref(false);

let copyTimer: ReturnType<typeof setTimeout> | null = null;

/** 平台名是专有名词，不做翻译；架构代号同理 */
const PLATFORM_NAMES: Record<string, string> = {
  macos: 'macOS',
  windows: 'Windows',
  linux: 'Linux',
  ios: 'iOS',
  android: 'Android'
};

const platformName = PLATFORM_NAMES[platform()] ?? platform();
const archName = arch();

const techStack: Array<{ name: string; version: string; icon: IconName; color: string }> = [
  { name: 'Tauri', version: 'v2.10.1', icon: 'tauri', color: '#26b8c9' },
  { name: 'Vue', version: 'v3.5.30', icon: 'vue', color: '#3fa678' },
  { name: 'Element Plus', version: 'v2.13.5', icon: 'elementPlus', color: '#3f8fd6' },
  { name: 'TypeScript', version: 'v5.9.3', icon: 'typescript', color: '#3178c6' }
];

const copyVersion = async () => {
  try {
    await clipboard.writeText(version.value);
    versionCopied.value = true;
    if (copyTimer) {
      clearTimeout(copyTimer);
    }
    copyTimer = setTimeout(() => {
      versionCopied.value = false;
    }, 1400);
    ElMessage({ type: 'success', message: t('about.versionCopied'), duration: 2000, grouping: true });
  } catch (error) {
    console.error('Failed to copy version:', error);
  }
};

onMounted(async () => {
  try {
    version.value = await getVersion();
  } catch (error) {
    console.error('Failed to get version:', error);
    version.value = '—';
  }
});

onUnmounted(() => {
  if (copyTimer) {
    clearTimeout(copyTimer);
    copyTimer = null;
  }
});
</script>

<style scoped lang="scss">
.about-page {
  display: flex;
  flex-direction: column;
  gap: var(--s-4);
}

/* ── 标识 ──────────────────────────────────────────────────── */
.hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--s-6) var(--s-5) var(--s-5);
  gap: 2px;
}

.logo-wrap {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 84px;
  height: 84px;
  margin-bottom: var(--s-4);
  border-radius: var(--r-xl);
  background: var(--surface);
  border: 1px solid var(--border);
  box-shadow: var(--shadow-2);

  /* 商标背后一盏暖光，给这页一个视觉落点 */
  &::before {
    content: '';
    position: absolute;
    inset: -18px;
    border-radius: 50%;
    background: radial-gradient(circle, var(--glow-a) 0%, transparent 70%);
    filter: blur(10px);
    z-index: -1;
  }
}

.app-name {
  font-size: var(--fs-xl);
  font-weight: 600;
  letter-spacing: -0.015em;
  color: var(--text);
}

.developer {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: var(--fs-sm);
  color: var(--text-3);
}

.developer-icon {
  color: var(--text-3);
}

/* ── 键值列表 ──────────────────────────────────────────────── */
.info-list {
  display: flex;
  flex-direction: column;
  gap: var(--s-1);
  padding: var(--s-2) var(--s-5);
}

.info-row {
  display: flex;
  align-items: center;
  gap: var(--s-3);
  width: 100%;
  min-height: 40px;
  padding: var(--s-2) 0;
  border: 0;
  background: transparent;
  text-align: left;
  color: inherit;

  & + .info-row {
    border-top: 1px solid var(--border);
  }

  &.is-action {
    cursor: pointer;
    border-radius: var(--r-sm);
    transition: background var(--dur-1) var(--ease-standard);

    &:hover {
      background: var(--surface-2);
    }

    &:hover .info-copy {
      opacity: 1;
    }
  }
}

.info-label {
  flex: 1;
  font-size: var(--fs-md);
  color: var(--text-2);
}

.info-value {
  font-size: var(--fs-md);
  color: var(--text);
}

.info-copy {
  color: var(--text-3);
  opacity: 0.4;
  transition: opacity var(--dur-1) var(--ease-standard),
    color var(--dur-1) var(--ease-standard);

  &.is-copied {
    color: var(--success);
    opacity: 1;
  }
}

/* ── 技术栈 ────────────────────────────────────────────────── */
.tech-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--s-2);
  padding: var(--s-3);
}

.tech-item {
  display: flex;
  align-items: center;
  gap: var(--s-3);
  padding: var(--s-3);
  border: 1px solid var(--border);
  border-radius: var(--r-md);
  background: var(--surface-2);
  transition: border-color var(--dur-1) var(--ease-standard),
    background var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--surface-3);
    border-color: var(--border-2);
  }
}

.tech-mark {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  flex-shrink: 0;
  border-radius: var(--r-sm);
  background: var(--surface);
  border: 1px solid var(--border);
}

.tech-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.tech-name {
  font-size: var(--fs-md);
  font-weight: 500;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tech-version {
  font-size: var(--fs-2xs);
  color: var(--text-3);
}

/* ── 版权 ──────────────────────────────────────────────────── */
.copyright {
  padding: var(--s-4) 0 0;
  text-align: center;
  font-size: var(--fs-xs);
  color: var(--text-3);
  line-height: 1.6;
}

@media (max-width: 480px) {
  .tech-grid {
    grid-template-columns: 1fr;
  }
}
</style>
