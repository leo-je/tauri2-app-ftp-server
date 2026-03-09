<template>
  <div class="about-container">
    <div class="about-content">
      <!-- Hero Section -->
      <div :class="['hero-section', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.1s;' : ''">
        <div class="logo-wrapper">
          <AppLogo :size="96" />
        </div>
        <h1 class="app-name">{{ $t('about.appName') }}</h1>
        <div
          class="version-badge"
          @click="copyVersion"
          :title="$t('about.clickToCopy')"
        >
          <SvgIcon name="copy" :size="16" class="version-icon" />
          <span class="version-text">{{ $t('about.version') }} {{ version }}</span>
          <SvgIcon name="copy" :size="14" class="copy-icon" />
        </div>
        <p class="developer-info">
          <SvgIcon name="user" :size="16" class="developer-icon" />
          <span>{{ $t('about.developer') }}: {{ $t('about.developerName') }}</span>
        </p>
      </div>

      <!-- Tech Stack Section -->
      <div :class="['tech-stack-card', 'about-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.2s;' : ''">
        <div class="card-header">
          <SvgIcon name="terminal" :size="20" class="card-icon" />
          <span>{{ $t('about.techStack') }}</span>
        </div>
        <div class="tech-grid">
          <div
            v-for="tech in techStack"
            :key="tech.name"
            class="tech-item"
          >
            <div class="tech-icon-wrapper" :style="{ background: tech.gradient }">
              <SvgIcon :name="tech.icon" :size="24" class="tech-icon" />
            </div>
            <span class="tech-name">{{ tech.name }}</span>
            <span class="tech-version">{{ tech.version }}</span>
          </div>
        </div>
      </div>

      <!-- Features Section - Hidden temporarily -->
      <!-- <div :class="['features-card', 'about-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.3s;' : ''">
        <div class="card-header">
          <SvgIcon name="checkCircle" :size="20" class="card-icon" />
          <span>{{ $t('about.features') }}</span>
        </div>
        <div class="features-list">
          <div
            v-for="(feature, key) in featuresData"
            :key="key"
            class="feature-item"
          >
            <div class="feature-icon-wrapper">
              <SvgIcon :name="feature.icon" :size="20" class="feature-icon" />
            </div>
            <div class="feature-content">
              <h3 class="feature-title">{{ $t(`about.featuresList.${key}.title`) }}</h3>
              <p class="feature-desc">{{ $t(`about.featuresList.${key}.desc`) }}</p>
            </div>
          </div>
        </div>
      </div> -->

      <!-- Links Section - Hidden temporarily -->
      <!-- <div :class="['links-card', 'about-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.4s;' : ''">
        <div class="card-header">
          <SvgIcon name="link" :size="20" class="card-icon" />
          <span>{{ $t('about.linksSection') }}</span>
        </div>
        <div class="links-grid">
          <div
            v-for="link in links"
            :key="link.name"
            class="link-item"
            @click="openLink(link.url)"
          >
            <SvgIcon :name="link.icon" :size="20" class="link-icon" />
            <span class="link-text">{{ $t(`about.links.${link.name}`) }}</span>
            <SvgIcon name="arrowDown" :size="16" class="link-arrow" />
          </div>
        </div>
      </div> -->

      <!-- Copyright Footer -->
      <div :class="['copyright-section', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.5s;' : ''">
        <p class="copyright-text">{{ $t('about.copyright') }}</p>
        <p class="rights-text">{{ $t('about.rights') }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { getVersion } from '@tauri-apps/api/app';
import { openUrl } from '@tauri-apps/plugin-opener';
import AppLogo from '../components/AppLogo.vue';
import { SvgIcon, type IconName } from '../components/icons';
import clipboard from 'tauri-plugin-clipboard-api';

const { t } = useI18n();
const version = ref('0.1.0');
const isFirstLoad = ref(true);

// Tech stack data
const techStack = ref<Array<{ name: string; version: string; icon: IconName; gradient: string }>>([
  {
    name: 'Tauri',
    version: 'v2.2.0',
    icon: 'server',
    gradient: 'linear-gradient(135deg, #24C8DB 0%, #14B8A6 100%)'
  },
  {
    name: 'Vue',
    version: 'v3.5.13',
    icon: 'terminal',
    gradient: 'linear-gradient(135deg, #42B883 0%, #35495E 100%)'
  },
  {
    name: 'Element Plus',
    version: 'v2.9.5',
    icon: 'checkCircle',
    gradient: 'linear-gradient(135deg, #409EFF 0%, #66B1FF 100%)'
  },
  {
    name: 'TypeScript',
    version: 'v5.7.3',
    icon: 'terminal',
    gradient: 'linear-gradient(135deg, #3178C6 0%, #235A97 100%)'
  }
]);

// Features data
const featuresData = ref<Record<string, { icon: IconName }>>({
  crossPlatform: { icon: 'global' },
  easyConfig: { icon: 'link' },
  permissionControl: { icon: 'lock' },
  realtime: { icon: 'clock' }
});

// Links data
const links = ref<Array<{ name: string; icon: IconName; url: string }>>([
  { name: 'github', icon: 'link', url: 'https://github.com/your-repo/ftp-server' },
  { name: 'documentation', icon: 'link', url: 'https://github.com/your-repo/ftp-server#readme' },
  { name: 'feedback', icon: 'link', url: 'https://github.com/your-repo/ftp-server/issues' },
  { name: 'changelog', icon: 'link', url: 'https://github.com/your-repo/ftp-server/releases' }
]);

// Copy version to clipboard
const copyVersion = async () => {
  try {
    await clipboard.writeText(version.value);
    ElMessage({
      type: 'success',
      message: t('about.versionCopied'),
      duration: 2000
    });
  } catch (error) {
    console.error('Failed to copy version:', error);
  }
};

// Open external link
const openLink = async (url: string) => {
  try {
    await openUrl(url);
  } catch (error) {
    console.error('Failed to open link:', error);
  }
};

onMounted(async () => {
  // Get app version
  try {
    version.value = await getVersion();
  } catch (error) {
    console.error('Failed to get version:', error);
    version.value = '0.1.0';
  }

  // Remove first load animation after completion
  nextTick(() => {
    setTimeout(() => {
      isFirstLoad.value = false;
    }, 600);
  });
});
</script>

<style lang="scss" scoped>
.about-container {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 20px;
  overflow: auto;
}

.about-content {
  width: 100%;
  max-width: 600px;
}

/* Hero Section */
.hero-section {
  text-align: center;
  padding: 32px 0;
  margin-bottom: 24px;

  .logo-wrapper {
    width: 96px;
    height: 96px;
    margin: 0 auto 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.15);
    border-radius: 20px;
    backdrop-filter: blur(10px);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  }

  .app-name {
    font-size: 28px;
    font-weight: 700;
    color: #303133;
    margin: 0 0 16px;
    letter-spacing: 0.5px;
  }

  .version-badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
    border: 2px solid rgba(102, 126, 234, 0.2);
    border-radius: 24px;
    cursor: pointer;
    transition: all 0.3s ease;
    margin-bottom: 12px;

    &:hover {
      background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
      border-color: rgba(102, 126, 234, 0.4);
      transform: scale(1.05);

      .copy-icon {
        opacity: 1;
      }
    }

    .version-icon,
    .copy-icon {
      width: 16px;
      height: 16px;
      color: var(--primary-color);
    }

    .copy-icon {
      opacity: 0;
      transition: opacity 0.3s ease;
    }

    .version-text {
      font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
      font-size: 14px;
      font-weight: 600;
      color: #606266;
    }
  }

  .developer-info {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 14px;
    color: #909399;
    margin: 0;

    .developer-icon {
      width: 16px;
      height: 16px;
      color: var(--primary-color);
    }
  }
}

html.dark .hero-section {
  .app-name {
    color: #e0e0e0;
  }

  .version-badge {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.2) 0%, rgba(118, 75, 162, 0.2) 100%);
    border-color: rgba(102, 126, 234, 0.3);

    .version-text {
      color: #b0b0b0;
    }
  }

  .developer-info {
    color: #909399;
  }
}

/* Card Styles */
.about-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: var(--radius-lg);
  padding: 24px;
  margin-bottom: 20px;
  box-shadow: 0 4px 20px rgba(102, 126, 234, 0.15);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(102, 126, 234, 0.1);
  transition: all 0.3s ease;

  &:hover {
    box-shadow: 0 8px 32px rgba(102, 126, 234, 0.2);
    transform: translateY(-2px);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 20px;
    font-size: 16px;
    font-weight: 600;
    color: #303133;

    .card-icon {
      width: 20px;
      height: 20px;
      color: var(--primary-color);
    }
  }
}

html.dark .about-card {
  background: rgba(30, 30, 40, 0.95);
  border-color: rgba(102, 126, 234, 0.2);

  .card-header {
    color: #e0e0e0;
  }
}

/* Tech Stack Grid */
.tech-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.tech-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
  border-radius: var(--radius-md);
  border: 1px solid rgba(102, 126, 234, 0.1);
  transition: all 0.3s ease;

  &:hover {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
    border-color: rgba(102, 126, 234, 0.3);
    transform: translateY(-4px);
  }

  .tech-icon-wrapper {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);

    .tech-icon {
      width: 24px;
      height: 24px;
      color: white;
    }
  }

  .tech-name {
    font-size: 13px;
    font-weight: 600;
    color: #303133;
  }

  .tech-version {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 11px;
    color: #909399;
  }
}

html.dark .tech-item {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  border-color: rgba(102, 126, 234, 0.2);

  .tech-name {
    color: #e0e0e0;
  }

  .tech-version {
    color: #909399;
  }
}

/* Features List */
.features-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.feature-item {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 16px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
  border-radius: var(--radius-md);
  border: 1px solid rgba(102, 126, 234, 0.1);
  transition: all 0.3s ease;

  &:hover {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
    border-color: rgba(102, 126, 234, 0.3);
    transform: translateX(4px);
  }

  .feature-icon-wrapper {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: var(--gradient-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);

    .feature-icon {
      width: 20px;
      height: 20px;
      color: white;
    }
  }

  .feature-content {
    flex: 1;

    .feature-title {
      font-size: 15px;
      font-weight: 600;
      color: #303133;
      margin: 0 0 4px;
    }

    .feature-desc {
      font-size: 13px;
      color: #606266;
      margin: 0;
      line-height: 1.5;
    }
  }
}

html.dark .feature-item {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  border-color: rgba(102, 126, 234, 0.2);

  .feature-title {
    color: #e0e0e0;
  }

  .feature-desc {
    color: #b0b0b0;
  }
}

/* Links Grid */
.links-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
  border-radius: var(--radius-md);
  border: 1px solid rgba(102, 126, 234, 0.1);
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
    border-color: rgba(102, 126, 234, 0.3);
    transform: translateY(-2px);

    .link-arrow {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .link-icon {
    width: 20px;
    height: 20px;
    color: var(--primary-color);
    flex-shrink: 0;
  }

  .link-text {
    flex: 1;
    font-size: 14px;
    font-weight: 500;
    color: #303133;
  }

  .link-arrow {
    width: 16px;
    height: 16px;
    color: var(--primary-color);
    opacity: 0;
    transform: translateX(-4px);
    transition: all 0.3s ease;
  }
}

html.dark .link-item {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  border-color: rgba(102, 126, 234, 0.2);

  .link-text {
    color: #e0e0e0;
  }
}

/* Copyright Section */
.copyright-section {
  text-align: center;
  padding: 24px 0;
  margin-top: 20px;

  .copyright-text {
    font-size: 13px;
    color: #909399;
    margin: 0 0 4px;
  }

  .rights-text {
    font-size: 12px;
    color: #C0C4CC;
    margin: 0;
  }
}

html.dark .copyright-section {
  .copyright-text {
    color: #909399;
  }

  .rights-text {
    color: #6C6C6C;
  }
}

/* Fade-in Animation */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.fade-in {
  animation: fadeIn 0.6s ease forwards;
  opacity: 0;
}

/* Responsive */
@media (max-width: 480px) {
  .tech-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .links-grid {
    grid-template-columns: 1fr;
  }
}
</style>
