<template>
  <div class="auth-page">
    <!-- 运行中所有权限设置都被锁定，与其逐处解释，不如一次性说清 -->
    <div v-if="isServerRunning" class="notice">
      <SvgIcon name="lock" :size="14" class="notice-icon" />
      <span>{{ $t('auth.serverRunning') }}</span>
    </div>

    <!-- ── 常规设置 ──────────────────────────────────────────────── -->
    <section class="card">
      <div class="card-head">
        <SvgIcon name="settings" :size="16" class="card-head-icon" />
        <span class="card-title">{{ $t('auth.settings') }}</span>
      </div>
      <div class="card-body setting-list">
        <div class="setting-row">
          <div class="setting-text">
            <span class="setting-label">{{ $t('auth.autoStart') }}</span>
            <span class="setting-desc">{{ $t('auth.autoStartDesc') }}</span>
          </div>
          <el-switch v-model="autoStart" class="auth-switch" @change="onAutoStartChange" />
        </div>
        <div class="setting-row">
          <div class="setting-text">
            <span class="setting-label">{{ $t('auth.hideOnStartup') }}</span>
            <span class="setting-desc">{{ $t('auth.hideOnStartupDesc') }}</span>
          </div>
          <el-switch
            v-model="hideOnStartup"
            class="auth-switch"
            @change="() => persist('hideOnStartup', hideOnStartup)"
          />
        </div>
      </div>
    </section>

    <!-- ── 访问控制 ──────────────────────────────────────────────── -->
    <section class="card">
      <div class="card-head">
        <SvgIcon name="lock" :size="16" class="card-head-icon" />
        <span class="card-title">{{ $t('auth.accessControl') }}</span>
      </div>
      <div class="card-body setting-list">
        <div class="setting-row">
          <div class="setting-text">
            <span class="setting-label">{{ $t('auth.anonymous') }}</span>
            <span class="setting-desc">{{ $t('auth.anonymousDesc') }}</span>
          </div>
          <el-switch
            v-model="isAnonymous"
            class="auth-switch"
            :disabled="isServerRunning"
            @change="() => persist('isAnonymous', isAnonymous)"
          />
        </div>

        <transition name="reveal">
          <div v-if="isAnonymous" class="setting-row is-sub">
            <div class="setting-text">
              <span class="setting-label">{{ $t('auth.filePermission') }}</span>
              <span class="setting-desc">{{ $t('auth.filePermissionDesc') }}</span>
            </div>
            <el-radio-group
              v-model="fileAuth"
              class="segmented"
              :disabled="isServerRunning"
              @change="() => persist('fileauth', fileAuth)"
            >
              <el-radio-button value="R">{{ $t('auth.readOnly') }}</el-radio-button>
              <el-radio-button value="W">{{ $t('auth.readWrite') }}</el-radio-button>
            </el-radio-group>
          </div>
        </transition>
      </div>
    </section>

    <!-- ── 用户列表 ──────────────────────────────────────────────── -->
    <transition name="reveal">
      <section v-if="!isAnonymous" class="card">
        <div class="card-head">
          <SvgIcon name="users" :size="16" class="card-head-icon" />
          <span class="card-title">{{ $t('auth.userList') }}</span>
          <span class="chip mono">{{ tableData.length }}</span>
          <div class="card-head-actions">
            <el-button type="primary" size="small" :disabled="isServerRunning" @click="openAddUser">
              <SvgIcon name="plus" :size="14" class="btn-icon" />
              {{ $t('auth.addUser') }}
            </el-button>
          </div>
        </div>

        <div v-if="tableData.length === 0" class="empty-state">
          <SvgIcon name="users" :size="26" class="empty-icon" />
          <span class="empty-title">{{ $t('auth.noUsers') }}</span>
          <span class="empty-hint">{{ $t('auth.addUserHint') }}</span>
        </div>

        <div v-else class="card-body user-list">
          <div v-for="(user, index) in tableData" :key="index" class="user-row">
            <div class="user-main">
              <span class="user-name">{{ user.username }}</span>
              <span class="user-password mono">{{ user.password }}</span>
            </div>
            <span class="chip" :class="user.fileAuth === 'W' ? 'is-accent' : ''">
              {{ user.fileAuth === 'W' ? $t('auth.readWrite') : $t('auth.readOnly') }}
            </span>
            <div class="user-actions">
              <button
                type="button"
                class="icon-btn"
                :disabled="isServerRunning"
                :title="$t('auth.edit')"
                :aria-label="$t('auth.edit')"
                @click="editUser(index, user)"
              >
                <SvgIcon name="edit" :size="14" />
              </button>
              <button
                type="button"
                class="icon-btn is-danger"
                :disabled="isServerRunning"
                :title="$t('auth.delete')"
                :aria-label="$t('auth.delete')"
                @click="deleteUser(index, user)"
              >
                <SvgIcon name="delete" :size="14" />
              </button>
            </div>
          </div>
        </div>
      </section>
    </transition>

    <!-- ── IP 黑名单 ─────────────────────────────────────────────── -->
    <section class="card">
      <div class="card-head">
        <SvgIcon name="shield" :size="16" class="card-head-icon" />
        <span class="card-title">{{ $t('auth.ipBlacklist') }}</span>
        <span class="chip mono">{{ blacklistIps.length }}</span>
        <div class="card-head-actions">
          <el-button type="primary" size="small" :disabled="isServerRunning" @click="openAddIpDialog">
            <SvgIcon name="plus" :size="14" class="btn-icon" />
            {{ $t('auth.addIp') }}
          </el-button>
        </div>
      </div>

      <div class="card-body">
        <div v-if="blacklistIps.length === 0" class="empty-state is-compact">
          <span class="empty-hint">{{ $t('auth.noBlacklistIps') }}</span>
        </div>
        <div v-else class="ip-tags">
          <el-tag
            v-for="ip in blacklistIps"
            :key="ip"
            class="ip-tag"
            :closable="!isServerRunning"
            :disable-transitions="true"
            @close="removeIp(ip)"
          >
            {{ ip }}
          </el-tag>
        </div>
      </div>
    </section>

    <!-- ── 用户编辑抽屉 ──────────────────────────────────────────── -->
    <el-drawer
      v-model="drawer"
      :title="form.index !== undefined ? $t('auth.editUser') : $t('auth.addUser')"
      :direction="direction"
      size="360px"
      append-to-body
    >
      <el-form :model="form" class="drawer-form" label-position="top">
        <el-form-item :label="$t('auth.username')">
          <el-input v-model="form.username" :placeholder="$t('auth.usernamePlaceholder')" class="selectable">
            <template #prefix>
              <SvgIcon name="user" :size="14" />
            </template>
          </el-input>
        </el-form-item>
        <el-form-item :label="$t('auth.password')">
          <el-input v-model="form.password" :placeholder="$t('auth.passwordPlaceholder')" class="selectable">
            <template #prefix>
              <SvgIcon name="lock" :size="14" />
            </template>
          </el-input>
        </el-form-item>
        <el-form-item :label="$t('auth.permission')">
          <el-radio-group v-model="form.fileAuth" class="segmented is-wide">
            <el-radio-button value="R">{{ $t('auth.readOnly') }}</el-radio-button>
            <el-radio-button value="W">{{ $t('auth.readWrite') }}</el-radio-button>
          </el-radio-group>
        </el-form-item>
      </el-form>

      <div class="drawer-footer">
        <el-button @click="drawer = false">{{ $t('auth.cancel') }}</el-button>
        <el-button type="primary" @click="saveForm">{{ $t('auth.save') }}</el-button>
      </div>
    </el-drawer>
  </div>
</template>

<script lang="ts" setup>
import { onBeforeMount, reactive, ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import {
  ElSwitch,
  ElButton,
  ElDrawer,
  ElInput,
  ElMessage,
  ElMessageBox,
  ElRadioGroup,
  ElRadioButton,
  ElForm,
  ElFormItem,
  ElTag
} from 'element-plus'
import type { DrawerProps } from 'element-plus'

import store, { runtimeState, type StoreValue } from '../store'
import { SvgIcon } from '../components/icons'
import { validateUser } from '../utils/validation'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'

const { t } = useI18n()

interface TableDataItem {
  username: string;
  password: string;
  fileAuth: string;
  index: number | undefined;
}

const isAnonymous = ref(true)
const tableData = ref<TableDataItem[]>([])
const fileAuth = ref('R')
const autoStart = ref(true)
const hideOnStartup = ref(true)
const blacklistIps = ref<string[]>([])

const isServerRunning = computed(() => runtimeState.isServerRunning.value)

const init = async () => {
  try {
    const anonymous = await store.get('isAnonymous')
    isAnonymous.value = !!anonymous

    const storedFileAuth = await store.get('fileauth')
    fileAuth.value = storedFileAuth ? `${storedFileAuth}` : 'R'

    const users = await store.get('tableData')
    tableData.value = users && Array.isArray(users) && users.length > 0 ? (users as TableDataItem[]) : []

    try {
      autoStart.value = await isEnabled()
    } catch (e) {
      console.warn('Failed to get autostart status:', e)
      autoStart.value = true
    }

    // 缺省必须是 true：插件在键不存在时返回 undefined（不是 null），
    // 而 lib.rs 启动时确实会隐藏窗口——显示成 false 就和实际行为正好相反
    const hs = await store.get('hideOnStartup')
    hideOnStartup.value = hs == null ? true : !!hs

    await loadBlacklistIps()
  } catch (e) {
    console.error(e)
    tableData.value = []
    void persist('tableData', [])
  }
}

/** 统一收口：store 写入是异步的，不 await 的话失败会被上报成成功 */
const persist = async <K extends keyof StoreValue>(key: K, value: StoreValue[K]) => {
  try {
    await store.set(key, value)
  } catch (e) {
    console.error(`Failed to persist "${key}":`, e)
    ElMessage.error(t('auth.saveFailed'))
  }
}

const onAutoStartChange = async () => {
  try {
    if (autoStart.value) {
      await enable()
    } else {
      await disable()
    }
  } catch (e) {
    console.error('Failed to toggle autostart:', e)
    // 让开关回到真实状态，而不是停在一个没生效的位置
    autoStart.value = !autoStart.value
  }
}

onBeforeMount(() => {
  init()
})

const form = reactive<TableDataItem>({ username: '', password: '', index: undefined, fileAuth: 'R' })
const drawer = ref(false)
const direction = ref<DrawerProps['direction']>('rtl')

const openAddUser = () => {
  form.username = ''
  form.password = ''
  form.index = undefined
  form.fileAuth = 'R'
  drawer.value = true
}

const editUser = (index: number, user: TableDataItem) => {
  form.index = index
  form.username = user.username
  form.password = user.password
  form.fileAuth = user.fileAuth
  drawer.value = true
}

const deleteUser = (index: number, user: TableDataItem) => {
  ElMessageBox.confirm(
    t('auth.deleteConfirm', { username: user.username }),
    t('auth.deleteTitle'),
    {
      confirmButtonText: t('auth.confirm'),
      cancelButtonText: t('auth.cancel'),
      type: 'warning'
    }
  )
    .then(async () => {
      tableData.value.splice(index, 1)
      await persist('tableData', tableData.value)
      ElMessage.success(t('auth.userDeleted'))
    })
    .catch(() => {
      // 用户取消删除
    })
}

const loadBlacklistIps = async () => {
  try {
    // 后端那份黑名单是内存态，进程一重启就空了。
    // 先把持久化的副本推回去，再读权威列表——否则界面上看着有，实际不生效。
    const saved = await store.get('blacklistIps')
    if (Array.isArray(saved) && saved.length > 0) {
      await invoke('load_blacklist', { ips: saved })
    }

    const ips = await invoke<string[]>('get_blacklist_ips')
    blacklistIps.value = ips || []
  } catch (e) {
    console.error('Failed to load blacklist:', e)
    blacklistIps.value = []
  }
}

/**
 * 与后端 `IpBlacklist::normalize_entry` 对齐的轻量校验：
 * 接受单个 IP 或 CIDR 网段，前缀范围按地址族区分（IPv4 32 / IPv6 128）。
 * 只拦明显不合法的写法，最终仍以后端结果为准。
 */
const validateIpEntry = (value: string): string | true => {
  const entry = value.trim()
  if (!entry) {
    return t('auth.ipRequired')
  }

  const slash = entry.indexOf('/')
  const addr = (slash === -1 ? entry : entry.slice(0, slash)).trim()
  const prefixText = slash === -1 ? '' : entry.slice(slash + 1).trim()
  const isIpv6 = addr.includes(':')

  const addrValid = isIpv6
    ? /^[0-9a-fA-F:]{2,}$/.test(addr)
    : /^\d{1,3}(\.\d{1,3}){3}$/.test(addr) &&
      addr.split('.').every((part) => Number(part) <= 255)

  if (!addrValid) {
    return t('auth.ipInvalidFormat')
  }

  if (slash !== -1) {
    const max = isIpv6 ? 128 : 32
    if (!/^\d{1,3}$/.test(prefixText) || Number(prefixText) > max) {
      return t('auth.ipPrefixRange', { max })
    }
  }

  return true
}

const openAddIpDialog = () => {
  ElMessageBox.prompt(
    t('auth.ipInputPlaceholder'),
    t('auth.addIp'),
    {
      confirmButtonText: t('auth.confirm'),
      cancelButtonText: t('auth.cancel'),
      inputValue: '',
      inputValidator: (value: string) => validateIpEntry(value)
    }
  )
    .then(async ({ value }) => {
      try {
        await invoke('add_ip_to_blacklist', { ip: value.trim() })
        await loadBlacklistIps()
        await persist('blacklistIps', blacklistIps.value)
        ElMessage.success(t('auth.ipAdded'))
      } catch (e) {
        console.error('Failed to add IP:', e)
        ElMessage.error(String(e))
      }
    })
    .catch(() => {
      // 用户取消
    })
}

const removeIp = async (ip: string) => {
  try {
    await invoke('remove_ip_from_blacklist', { ip })
    await loadBlacklistIps()
    await persist('blacklistIps', blacklistIps.value)
    ElMessage.success(t('auth.ipRemoved'))
  } catch (e) {
    console.error('Failed to remove IP:', e)
    ElMessage.error(String(e))
  }
}

const saveForm = async () => {
  const validation = validateUser({
    username: form.username,
    password: form.password,
    fileAuth: form.fileAuth
  })

  if (validation.errors.length > 0) {
    ElMessage({ type: 'error', message: validation.errors.join('；') })
    return
  }

  if (validation.warnings.length > 0) {
    validation.warnings.forEach((warning) => {
      ElMessage({ type: 'warning', message: warning, duration: 5000 })
    })
  }

  drawer.value = false
  if (form.index != null) {
    tableData.value[form.index] = Object.assign({}, form)
  } else {
    tableData.value.push(Object.assign({}, form))
  }
  await persist('tableData', tableData.value)
  ElMessage.success(form.index !== undefined ? t('auth.userUpdated') : t('auth.userAdded'))
}
</script>

<style lang="scss" scoped>
.auth-page {
  display: flex;
  flex-direction: column;
  gap: var(--s-4);
}

.notice {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  padding: 9px var(--s-3);
  border: 1px solid var(--accent-line);
  border-radius: var(--r-md);
  background: var(--accent-soft);
  color: var(--accent);
  font-size: var(--fs-sm);
  font-weight: 500;
}

.notice-icon {
  flex-shrink: 0;
}

/* ── 设置行 ─────────────────────────────────────────────────── */
.setting-list {
  display: flex;
  flex-direction: column;
  gap: var(--s-1);
  padding: var(--s-2) var(--s-5);
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--s-4);
  min-height: 56px;
  padding: var(--s-2) 0;

  & + .setting-row {
    border-top: 1px solid var(--border);
  }

  /* 子项：从属关系用左边距和浅色文字表达 */
  &.is-sub {
    padding-left: var(--s-3);
    border-left: 2px solid var(--border);
    margin-left: 2px;
  }
}

.setting-text {
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.setting-label {
  font-size: var(--fs-md);
  font-weight: 500;
  color: var(--text);
}

.setting-desc {
  font-size: var(--fs-sm);
  color: var(--text-3);
}

.auth-switch {
  flex-shrink: 0;
}

.segmented {
  flex-shrink: 0;

  :deep(.el-radio-button__inner) {
    padding: 5px 14px;
    font-size: var(--fs-sm);
  }

  /* 抽屉里让两个选项等分整行 */
  &.is-wide {
    width: 100%;

    :deep(.el-radio-button) {
      flex: 1;
    }

    :deep(.el-radio-button__inner) {
      width: 100%;
    }
  }
}

/* ── 用户列表 ───────────────────────────────────────────────── */
.user-list {
  display: flex;
  flex-direction: column;
  padding: 0;
  max-height: 288px;
  overflow-y: auto;
}

.user-row {
  display: flex;
  align-items: center;
  gap: var(--s-3);
  padding: 10px var(--s-5);
  transition: background var(--dur-1) var(--ease-standard);

  & + .user-row {
    border-top: 1px solid var(--border);
  }

  &:hover {
    background: var(--surface-2);
  }

  &:hover .user-actions {
    opacity: 1;
  }
}

.user-main {
  display: flex;
  flex-direction: column;
  gap: 1px;
  flex: 1;
  min-width: 0;
}

.user-name {
  font-size: var(--fs-md);
  font-weight: 500;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.user-password {
  font-size: var(--fs-xs);
  color: var(--text-3);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 行操作常显但压暗：藏到 hover 才出现的话，用户根本不知道有这两个按钮 */
.user-actions {
  display: flex;
  align-items: center;
  gap: 2px;
  flex-shrink: 0;
  opacity: 0.5;
  transition: opacity var(--dur-1) var(--ease-standard);

  &:focus-within {
    opacity: 1;
  }
}

/* ── 空状态 ────────────────────────────────────────────────── */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: var(--s-6) var(--s-5);
  text-align: center;

  &.is-compact {
    padding: var(--s-4) var(--s-5);
  }
}

.empty-icon {
  color: var(--text-3);
  opacity: 0.5;
  margin-bottom: 2px;
}

.empty-title {
  font-size: var(--fs-md);
  font-weight: 500;
  color: var(--text-2);
}

.empty-hint {
  font-size: var(--fs-sm);
  color: var(--text-3);
}

/* ── IP 标签 ───────────────────────────────────────────────── */
.ip-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--s-2);
}

.ip-tag {
  font-size: var(--fs-sm);
}

/* ── 抽屉 ──────────────────────────────────────────────────── */
.drawer-form {
  :deep(.el-form-item) {
    margin-bottom: var(--s-5);
  }

  :deep(.el-form-item__label) {
    padding-bottom: 6px;
    font-size: var(--fs-sm);
    font-weight: 500;
    line-height: 1.4;
  }
}

.drawer-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--s-2);
  padding-top: var(--s-5);
  margin-top: var(--s-5);
  border-top: 1px solid var(--border);
}

.btn-icon {
  margin-right: 4px;
}

.reveal-enter-active,
.reveal-leave-active {
  transition: opacity var(--dur-2) var(--ease-standard),
    transform var(--dur-2) var(--ease-out);
}

.reveal-enter-from,
.reveal-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
