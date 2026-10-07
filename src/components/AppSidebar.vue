<template>
  <aside class="sidebar">
    <nav class="nav">
      <RouterLink
        v-for="item in navItems"
        :key="item.name"
        :to="item.to"
        class="nav-item"
        :class="{ 'is-active': route.name === item.name }"
      >
        <span class="nav-rail" aria-hidden="true"></span>
        <SvgIcon :name="item.icon" :size="16" class="nav-icon" />
        <span class="nav-label">{{ $t(item.label) }}</span>
        <!-- 服务在跑的时候，从任何一页都能看见 -->
        <span
          v-if="item.name === 'server' && isServerRunning"
          class="led is-live nav-led"
          :title="$t('status.running')"
        ></span>
      </RouterLink>
    </nav>

    <div class="sidebar-foot">
      <ThemeToggle />
      <LanguageSwitcher />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { SvgIcon, type IconName } from './icons';
import ThemeToggle from './ThemeToggle.vue';
import LanguageSwitcher from './LanguageSwitcher.vue';
import { runtimeState } from '../store';

const route = useRoute();
const isServerRunning = runtimeState.isServerRunning;

const navItems: Array<{ name: string; to: string; icon: IconName; label: string }> = [
  { name: 'server', to: '/', icon: 'server', label: 'tabs.server' },
  { name: 'auth', to: '/auth', icon: 'shield', label: 'tabs.auth' },
  { name: 'about', to: '/about', icon: 'info', label: 'tabs.about' }
];
</script>

<style scoped lang="scss">
.sidebar {
  width: var(--rail-w);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  padding: var(--s-2) var(--s-2) var(--s-3);
  background: var(--chrome);
  border-right: 1px solid var(--border);
  z-index: 2;
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: var(--s-2);
  height: 34px;
  padding: 0 var(--s-3) 0 10px;
  border-radius: var(--r-md);
  text-decoration: none;
  color: var(--text-2);
  font-size: var(--fs-md);
  font-weight: 500;
  transition: background var(--dur-1) var(--ease-standard),
    color var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--chrome-hover);
    color: var(--text);
  }

  &.is-active {
    background: var(--accent-soft);
    color: var(--accent);
    font-weight: 600;
  }
}

/* 左侧的强调条：当前页唯一的「位置」提示 */
.nav-rail {
  position: absolute;
  left: 0;
  top: 50%;
  width: 2px;
  height: 0;
  border-radius: var(--r-full);
  background: var(--accent);
  transform: translateY(-50%);
  transition: height var(--dur-2) var(--ease-out);
}

.nav-item.is-active .nav-rail {
  height: 16px;
}

.nav-icon {
  flex-shrink: 0;
  opacity: 0.85;
}

.nav-item.is-active .nav-icon {
  opacity: 1;
}

.nav-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.nav-led {
  margin-left: auto;
  margin-right: 2px;
}

.sidebar-foot {
  display: flex;
  flex-direction: column;
  gap: var(--s-2);
  padding-top: var(--s-3);
  border-top: 1px solid var(--border);
}
</style>
