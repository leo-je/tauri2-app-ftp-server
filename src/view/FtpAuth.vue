<template>
    <div class="auth-container">
        <div class="auth-content">
            <!-- 标题区域 -->
            <div class="header-section fade-in" style="display: none;">
                <div class="icon-wrapper">
                    <SvgIcon name="lock" :size="24" />
                </div>
                <h1 class="ftp-title">权限管理</h1>
                <p class="ftp-subtitle">配置 FTP 服务器的访问权限</p>
            </div>

            <!-- 常规设置卡片 -->
            <div :class="['settings-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.05s;' : ''">
                <div class="card-header">
                    <SvgIcon name="settings" :size="20" class="card-icon" />
                    <span>{{ $t('auth.settings') }}</span>
                </div>
                <div class="settings-list">
                    <div class="access-mode-row">
                        <div class="mode-info">
                            <span class="mode-label">{{ $t('auth.autoStart') }}</span>
                            <span class="mode-desc">{{ $t('auth.autoStartDesc') }}</span>
                        </div>
                        <el-switch
                            v-model="autoStart"
                            @change="onAutoStartChange"
                            class="auth-switch"
                        />
                    </div>
                    <div class="access-mode-row setting-item">
                        <div class="mode-info">
                            <span class="mode-label">{{ $t('auth.hideOnStartup') }}</span>
                            <span class="mode-desc">{{ $t('auth.hideOnStartupDesc') }}</span>
                        </div>
                        <el-switch
                            v-model="hideOnStartup"
                            @change="() => store.set('hideOnStartup', hideOnStartup)"
                            class="auth-switch"
                        />
                    </div>
                </div>
            </div>

            <!-- 匿名访问设置卡片 -->
            <div :class="['auth-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.1s;' : ''">
                <div v-if="isServerRunning" class="readonly-banner">
                    <SvgIcon name="lock" :size="16" />
                    <span>{{ $t('auth.serverRunning') }}</span>
                </div>
                <div class="access-mode-row">
                    <div class="mode-info">
                        <span class="mode-label">{{ $t('auth.anonymous') }}</span>
                        <span class="mode-desc">{{ $t('auth.anonymousDesc') }}</span>
                    </div>
                    <el-switch
                        v-model="isAnonymous"
                        @change="() => store.set('isAnonymous', isAnonymous)"
                        class="auth-switch"
                        :disabled="isServerRunning"
                    />
                    <transition name="slide-fade">
                        <el-radio-group
                            v-if="isAnonymous"
                            v-model="fileAuth"
                            size="small"
                            @change="() => store.set('fileauth', fileAuth)"
                            class="inline-permission-radio"
                            :disabled="isServerRunning"
                        >
                            <el-radio-button label="R">{{ $t('auth.readOnly') }}</el-radio-button>
                            <el-radio-button label="W">{{ $t('auth.readWrite') }}</el-radio-button>
                        </el-radio-group>
                    </transition>
                </div>
            </div>

            <!-- 用户管理卡片 -->
            <transition name="slide-fade">
                <div v-if="!isAnonymous" :class="['users-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.2s;' : ''">
                    <div class="card-header">
                        <SvgIcon name="users" :size="20" class="card-icon" />
                        <span>{{ $t('auth.userList') }}</span>
                        <el-button
                            type="primary"
                            class="add-user-btn"
                            @click="openAddUser"
                            :disabled="isServerRunning"
                        >
                            <SvgIcon name="plus" :size="16" class="btn-icon" />
                            {{ $t('auth.addUser') }}
                        </el-button>
                    </div>

                    <div class="table-wrapper">
                        <ElTable
                            :data="tableData"
                            class="ftp-table"
                            :header-cell-style="{ background: 'transparent' }"
                            :row-class-name="getRowClassName"
                            height="300"
                        >
                            <ElTableColumn :label="$t('auth.username')" prop="username" min-width="100">
                                <template #default="scope">
                                    <div class="user-cell">
                                        <span>{{ scope.row.username }}</span>
                                    </div>
                                </template>
                            </ElTableColumn>
                            <ElTableColumn :label="$t('auth.password')" prop="password" min-width="120">
                                <template #default="scope">
                                    <div class="password-cell">

                                        <span class="password-dots">{{ scope.row.password }}</span>
                                    </div>
                                </template>
                            </ElTableColumn>
                            <el-table-column :label="$t('auth.actions')" width="120" fixed="right">
                                <template #default="scope">
                                    <div class="action-buttons">
                                        <el-button
                                            link
                                            type="primary"
                                            size="small"
                                            @click="editUser(scope)"
                                            class="action-btn edit-btn"
                                            :disabled="isServerRunning"
                                        >
                                            <SvgIcon name="edit" :size="14" class="action-icon" />
                                            {{ $t('auth.edit') }}
                                        </el-button>
                                        <el-button
                                            link
                                            type="danger"
                                            size="small"
                                            @click="deleteRow(scope)"
                                            class="action-btn delete-btn"
                                            :disabled="isServerRunning"
                                        >
                                            <SvgIcon name="delete" :size="14" class="action-icon" />
                                            {{ $t('auth.delete') }}
                                        </el-button>
                                    </div>
                                </template>
                            </el-table-column>
                        </ElTable>

                        <!-- 颜色说明 -->
                        <div class="color-legend">
                            <div class="legend-item">
                                <span class="legend-color read-color"></span>
                                <span>{{ $t('auth.readOnly') }}</span>
                            </div>
                            <div class="legend-item">
                                <span class="legend-color write-color"></span>
                                <span>{{ $t('auth.readWrite') }}</span>
                            </div>
                        </div>
                    </div>
    </div>
    </transition>

    <!-- IP黑名单设置卡片 -->
    <div :class="['ip-blacklist-card', 'ftp-card', { 'fade-in': isFirstLoad }]">
      <div class="card-header">
        <SvgIcon name="lock" :size="20" class="card-icon" />
        <span>{{ $t('auth.ipBlacklist') || 'IP黑名单' }}</span>
        <el-button
          type="primary"
          class="add-user-btn"
          @click="openAddIpDialog"
          :disabled="isServerRunning"
        >
          <SvgIcon name="plus" :size="16" class="btn-icon" />
          {{ $t('auth.addIp') || '添加IP' }}
        </el-button>
      </div>
      
      <div class="ip-list-container">
        <div v-if="blacklistIps.length === 0" class="empty-ip-list">
          <span class="empty-text">{{ $t('auth.noBlacklistIps') || '暂无黑名单IP' }}</span>
        </div>
        <div v-else class="ip-tags">
          <el-tag
            v-for="ip in blacklistIps"
            :key="ip"
            closable
            :disable-transitions="false"
            @close="removeIp(ip)"
            class="ip-tag"
            effect="dark"
            :disabled="isServerRunning"
          >
            {{ ip }}
          </el-tag>
        </div>
      </div>
    </div>

    <!-- 用户编辑抽屉 -->
            <el-drawer
                v-model="drawer"
                :title="form.index !== undefined ? $t('auth.editUser') : $t('auth.addUser')"
                :direction="direction"
                size="400px"
                class="user-drawer"
            >
                <el-form :model="form" class="drawer-form" label-width="80px">
                    <el-form-item :label="$t('auth.username')" class="drawer-form-item">
                        <el-input
                            v-model="form.username"
                            :placeholder="$t('auth.usernamePlaceholder')"
                            class="ftp-input"
                        >
                            <template #prefix>
                                <SvgIcon name="user" :size="16" />
                            </template>
                        </el-input>
                    </el-form-item>
                    <el-form-item :label="$t('auth.password')" class="drawer-form-item">
                        <el-input
                            v-model="form.password"
                            :placeholder="$t('auth.passwordPlaceholder')"
                            class="ftp-input"
                        >
                            <template #prefix>
                                <SvgIcon name="lock" :size="16" />
                            </template>
                        </el-input>
                    </el-form-item>
                    <el-form-item :label="$t('auth.permission')" class="drawer-form-item">
                        <el-radio-group v-model="form.fileAuth" size="large" class="drawer-permission-radio">
                            <el-radio-button label="R">{{ $t('auth.readOnly') }}</el-radio-button>
                            <el-radio-button label="W">{{ $t('auth.readWrite') }}</el-radio-button>
                        </el-radio-group>
                    </el-form-item>
                </el-form>
                <div class="drawer-footer">
                    <ElButton @click="drawer = false" class="cancel-btn">{{ $t('auth.cancel') }}</ElButton>
                    <ElButton type="success" @click="saveForm" class="save-btn">{{ $t('auth.save') }}</ElButton>
                </div>
            </el-drawer>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onBeforeMount, onMounted, reactive, ref, nextTick, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import {
  ElSwitch, ElTable, ElTableColumn,
  ElButton, ElDrawer, DrawerProps, ElInput, ElMessage, ElMessageBox,
  ElRadioGroup, ElRadioButton, ElForm, ElFormItem, ElTag
} from 'element-plus'
import type { TableColumnCtx } from 'element-plus'

import store, { runtimeState } from '../store';
import { SvgIcon } from '../components/icons';
import { validateUser } from '../utils/validation';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';

const { t } = useI18n()

interface TableDataItem {
    username: string;
    password: string;
    fileAuth: string;
    index: number | undefined;
}

// Element Plus 表格行 scope 类型
interface TableScope {
    $index: number;
    row: TableDataItem;
    column: TableColumnCtx<TableDataItem>;
}

const isAnonymous = ref(true)
const tableData = ref<TableDataItem[]>([])
const fileAuth = ref('R')
const isFirstLoad = ref(true)
const autoStart = ref(true)
const hideOnStartup = ref(true)

// IP黑名单
const blacklistIps = ref<string[]>([])
const ipInput = ref('')

// 服务运行状态
const isServerRunning = computed(() => runtimeState.isServerRunning.value)

const init = async () => {
    try {
        let a = await store.get('isAnonymous');
        isAnonymous.value = !!a

        let fa = await store.get('fileauth');
        fileAuth.value = fa ? fa + '' : 'R'

        let t: unknown = await store.get('tableData');
        console.log("tableData:" + t)
        tableData.value = (t && Array.isArray(t) && t.length > 0) ? t as TableDataItem[] : []

        // 加载自启动设置
        try {
            autoStart.value = await isEnabled()
        } catch (e) {
            console.warn('Failed to get autostart status:', e)
            autoStart.value = true
        }

    // 加载启动时隐藏主界面设置
    let hs = await store.get('hideOnStartup')
    hideOnStartup.value = hs !== null ? !!hs : true

    // 加载IP黑名单
    await loadBlacklistIps()
  } catch (e) {
    console.error(e)
    tableData.value = []
    store.set('tableData', []);
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
        // 恢复原状态
        autoStart.value = !autoStart.value
    }
}

onBeforeMount(() => {
    init()
})

onMounted(() => {
    // 首次加载后移除动画，避免切换 tabs 时重复触发
    nextTick(() => {
        setTimeout(() => {
            isFirstLoad.value = false;
        }, 600); // 等待动画完成
    });
})

const form = reactive<TableDataItem>({ username: '', password: '', index: undefined, fileAuth: 'R' })
const drawer = ref(false)
const direction = ref<DrawerProps['direction']>('rtl')

const openAddUser = () => {
    form.password = ''
    form.username = ''
    form.index = undefined
    form.fileAuth = 'R'
    drawer.value = true
}

const editUser = (scope: TableScope) => {
    form.index = scope.$index
    form.username = scope.row.username
    form.password = scope.row.password
    form.fileAuth = scope.row.fileAuth
    drawer.value = true
}

const deleteRow = (scope: TableScope) => {
    ElMessageBox.confirm(
        t('auth.deleteConfirm', { username: scope.row.username }),
        t('auth.deleteTitle'),
        {
            confirmButtonText: t('auth.confirm'),
            cancelButtonText: t('auth.cancel'),
            type: 'warning',
        }
    ).then(() => {
        tableData.value.splice(scope.$index, 1)
        store.set('tableData', tableData.value)
        ElMessage.success(t('auth.userDeleted'))
    }).catch(() => {
        // 用户取消删除
    })
}

const getRowClassName = ({ row }: { row: TableDataItem }) => {
    return row.fileAuth === 'W' ? 'write-row' : 'read-row'
}

// IP黑名单相关函数
const loadBlacklistIps = async () => {
  try {
    const ips = await invoke<string[]>('get_blacklist_ips')
    blacklistIps.value = ips || []
  } catch (e) {
    console.error('Failed to load blacklist:', e)
    blacklistIps.value = []
  }
}

const openAddIpDialog = () => {
  ipInput.value = ''
  ElMessageBox.prompt(
    t('auth.ipInputPlaceholder') || '请输入IP地址或CIDR范围（如 192.168.1.1 或 10.0.0.0/24）',
    t('auth.addIp') || '添加IP到黑名单',
    {
      confirmButtonText: t('auth.confirm') || '确认',
      cancelButtonText: t('auth.cancel') || '取消',
      inputValue: ipInput.value,
      inputValidator: (value: string) => {
        if (!value.trim()) {
          return t('auth.ipRequired') || 'IP地址不能为空'
        }
        return true
      }
    }
  ).then(async ({ value }) => {
    try {
      await invoke('add_ip_to_blacklist', { ip: value.trim() })
      await loadBlacklistIps()
      ElMessage.success(t('auth.ipAdded') || 'IP已添加到黑名单')
    } catch (e) {
      console.error('Failed to add IP:', e)
      ElMessage.error(String(e))
    }
  }).catch(() => {
    // 用户取消
  })
}

const removeIp = async (ip: string) => {
  try {
    await invoke('remove_ip_from_blacklist', { ip })
    await loadBlacklistIps()
    ElMessage.success(t('auth.ipRemoved') || 'IP已从黑名单移除')
  } catch (e) {
    console.error('Failed to remove IP:', e)
    ElMessage.error(String(e))
  }
}

const saveForm = () => {
    // 验证用户数据
    const validation = validateUser({
        username: form.username,
        password: form.password,
        fileAuth: form.fileAuth
    });

    // 显示错误
    if (validation.errors.length > 0) {
        ElMessage({ type: 'error', message: validation.errors.join('；') });
        return;
    }

    // 显示警告（但允许继续）
    if (validation.warnings.length > 0) {
        validation.warnings.forEach(warn => {
            ElMessage({ type: 'warning', message: warn, duration: 5000 });
        });
    }

    // 保存数据
    drawer.value = false;
    if (form.index != null) {
        tableData.value[form.index] = Object.assign({}, form)
    } else {
        tableData.value.push(Object.assign({}, form))
    }
    try {
        store.set('tableData', tableData.value)
        ElMessage.success(form.index !== undefined ? t('auth.userUpdated') : t('auth.userAdded'))
    } catch (e) {
        console.error('保存数据失败:', e)
        ElMessage.error(t('auth.saveFailed'))
    }
}

</script>

<style lang="scss" scoped>
.auth-container {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding: 20px;
    overflow: auto;
}

.auth-content {
    width: 100%;
    max-width: 800px;
}

.header-section {
    text-align: center;
    margin-bottom: 24px;
    
    .icon-wrapper {
        width: 56px;
        height: 56px;
        margin: 0 auto 16px;
        font-size: 24px;
    }
}

.auth-card,
.users-card {
    margin-bottom: 16px;
}

.settings-card {
    margin-bottom: 16px;

    .settings-list {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .setting-item {
        padding-top: 16px;
        border-top: 1px solid rgba(102, 126, 234, 0.1);
    }
}

html.dark .settings-card {
    .setting-item {
        border-top-color: rgba(102, 126, 234, 0.2);
    }
}

/* 访问模式紧凑行 */
.access-mode-row {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;

    .mode-info {
        display: flex;
        flex-direction: column;
        gap: 2px;

        .mode-label {
            font-size: 15px;
            font-weight: 600;
            color: #303133;
        }

        .mode-desc {
            font-size: 12px;
            color: #909399;
        }
    }

    .inline-permission-radio {
        margin-left: auto;

        :deep(.el-radio-button__inner) {
            padding: 6px 16px;
            border-radius: var(--radius-sm);
            font-size: 13px;
        }
    }
}

/* 只读状态提示横幅 */
.readonly-banner {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    margin-bottom: 12px;
    background: linear-gradient(135deg, rgba(245, 101, 101, 0.1) 0%, rgba(237, 137, 54, 0.1) 100%);
    border: 1px solid rgba(245, 101, 101, 0.2);
    border-radius: var(--radius-sm);
    font-size: 13px;
    color: #e53e3e;

    svg {
        width: 16px;
        height: 16px;
        color: #e53e3e;
    }
}

html.dark .access-mode-row {
    .mode-label {
        color: #e0e0e0;
    }
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
    
    .add-user-btn {
        margin-left: auto;
        border-radius: var(--radius-md);
        background: var(--gradient-primary);
        border: none;
        font-weight: 500;
        transition: all 0.3s ease;
        
        &:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }
        
        .btn-icon {
            width: 16px;
            height: 16px;
            margin-right: 4px;
        }
    }
}

html.dark .card-header {
    color: #e0e0e0;
}

.table-wrapper {
    position: relative;
    // min-height: 400px;
}

.ftp-table {
    border-radius: var(--radius-md);
    overflow: hidden;

    :deep(.el-table__header-wrapper) {
        th {
            background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
            color: #303133;
            font-weight: 600;
            font-size: 14px;
            border-bottom: 2px solid rgba(102, 126, 234, 0.1);
        }
    }

    :deep(.el-table__body-wrapper) {
        tr {
            transition: all 0.3s ease;

            td {
                border-bottom: 1px solid rgba(102, 126, 234, 0.05);
            }

            &.read-row {
                background: rgba(66, 153, 225, 0.15);
                border-left: 3px solid #4299e1;

                &:hover > td {
                    background: rgba(66, 153, 225, 0.25) !important;
                }
            }

            &.write-row {
                background: rgba(72, 187, 120, 0.15);
                border-left: 3px solid #48bb78;

                &:hover > td {
                    background: rgba(72, 187, 120, 0.25) !important;
                }
            }
        }
    }
}

html.dark .ftp-table {
    :deep(.el-table__header-wrapper) th {
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
        color: #e0e0e0;
        border-bottom-color: rgba(102, 126, 234, 0.2);
    }

    :deep(.el-table__body-wrapper) {
        td {
            border-bottom-color: rgba(102, 126, 234, 0.1);
        }

        tr.read-row {
            background: rgba(66, 153, 225, 0.2);
            border-left: 3px solid #4299e1;

            &:hover > td {
                background: rgba(66, 153, 225, 0.3) !important;
            }
        }

        tr.write-row {
            background: rgba(72, 187, 120, 0.2);
            border-left: 3px solid #48bb78;

            &:hover > td {
                background: rgba(72, 187, 120, 0.3) !important;
            }
        }
    }
}

.user-cell,
.password-cell {
    display: flex;
    align-items: center;
    gap: 8px;
    
    .user-icon,
    .lock-icon {
        width: 16px;
        height: 16px;
        color: var(--primary-color);
    }
}

.password-dots {
    font-family: monospace;
    letter-spacing: 2px;
}

/* 颜色说明 */
.color-legend {
    display: flex;
    gap: 24px;
    padding: 12px 16px;
    margin-top: 12px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: var(--radius-sm);
    font-size: 13px;
    color: #606266;

    .legend-item {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .legend-color {
        width: 16px;
        height: 16px;
        border-radius: 4px;

        &.read-color {
            background: rgba(66, 153, 225, 0.3);
            border-left: 3px solid #4299e1;
        }

        &.write-color {
            background: rgba(72, 187, 120, 0.3);
            border-left: 3px solid #48bb78;
        }
    }
}

html.dark .color-legend {
    background: rgba(255, 255, 255, 0.05);
    color: #b0b0b0;
}

.action-buttons {
    display: flex;
    gap: 8px;
}

.action-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    transition: all 0.3s ease;
    
    .action-icon {
        width: 14px;
        height: 14px;
    }
    
    &.edit-btn:hover {
        background: rgba(102, 126, 234, 0.1);
    }
    
    &.delete-btn:hover {
        background: rgba(245, 101, 101, 0.1);
    }
}

.empty-state {
    text-align: center;
    padding: 48px 24px;
    
    .empty-icon {
        width: 64px;
        height: 64px;
        color: #d0d0d0;
        margin-bottom: 16px;
    }
    
    .empty-text {
        font-size: 16px;
        font-weight: 500;
        color: #909399;
        margin-bottom: 8px;
    }
    
    .empty-hint {
        font-size: 14px;
        color: #c0c4cc;
    }
}

html.dark .empty-state {
    .empty-icon {
        color: #4a4a4a;
    }
}

/* IP黑名单卡片样式 */
.ip-blacklist-card {
  .ip-list-container {
    min-height: 60px;
  }

  .empty-ip-list {
    text-align: center;
    padding: 24px;
    color: #909399;
    font-size: 14px;
  }

  .ip-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    padding: 8px 0;

    .ip-tag {
      font-family: 'SFMono-Regular', monospace;
      font-size: 13px;
    }
  }
}

html.dark .ip-blacklist-card {
  .empty-ip-list {
    color: #a0a0a0;
  }
}

/* 抽屉样式 */
:deep(.user-drawer) {
    .el-drawer__header {
        margin-bottom: 0;
        padding: 20px 24px;
        border-bottom: 1px solid rgba(102, 126, 234, 0.1);
        
        .el-drawer__title {
            font-size: 18px;
            font-weight: 600;
            color: #303133;
        }
    }
    
    .el-drawer__body {
        padding: 24px;
    }
}

html.dark :deep(.user-drawer) {
    .el-drawer__header {
        border-bottom-color: rgba(102, 126, 234, 0.2);
        
        .el-drawer__title {
            color: #e0e0e0;
        }
    }
}

.drawer-form {
    .drawer-form-item {
        margin-bottom: 28px;

        :deep(.el-form-item__label) {
            font-weight: 500;
            color: #606266;
            margin-bottom: 10px;
            display: block;
        }

        :deep(.el-form-item__content) {
            display: block;
        }
    }
}

html.dark .drawer-form {
    :deep(.el-form-item__label) {
        color: #b0b0b0;
    }
}

.drawer-permission-radio {
    width: 100%;

    :deep(.el-radio-button__inner) {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 6px;
        padding: 10px 24px;
        border-radius: var(--radius-md);
        border: 2px solid rgba(102, 126, 234, 0.2);
        background: rgba(255, 255, 255, 0.8);
        color: #606266;
        font-weight: 500;
        transition: all 0.3s ease;

        &:hover {
            border-color: var(--primary-color);
            color: var(--primary-color);
        }
    }

    :deep(.el-radio-button__original-radio:checked + .el-radio-button__inner) {
        background: var(--gradient-primary);
        border-color: var(--primary-color);
        color: white;
        box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
    }
}

html.dark .drawer-permission-radio {
    :deep(.el-radio-button__inner) {
        background: rgba(30, 30, 40, 0.8);
        border-color: rgba(102, 126, 234, 0.3);
        color: #b0b0b0;
    }
}

.drawer-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding-top: 24px;
    border-top: 1px solid rgba(102, 126, 234, 0.1);
    
    .cancel-btn,
    .save-btn {
        min-width: 100px;
        border-radius: var(--radius-md);
        font-weight: 500;
        transition: all 0.3s ease;
    }
    
    .cancel-btn {
        &:hover {
            background: rgba(0, 0, 0, 0.05);
        }
    }
    
    .save-btn {
        background: var(--gradient-success);
        border: none;
        
        &:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(72, 198, 239, 0.3);
        }
    }
}

html.dark .drawer-footer {
    border-top-color: rgba(102, 126, 234, 0.2);
}

/* 过渡动画 */
.slide-fade-enter-active {
    transition: all 0.4s ease;
}

.slide-fade-leave-active {
    transition: all 0.3s ease;
}

.slide-fade-enter-from {
    transform: translateY(-20px);
    opacity: 0;
}

.slide-fade-leave-to {
    transform: translateY(20px);
    opacity: 0;
}

/* Element Plus 组件样式覆盖 */
:deep(.el-input) {
    .el-input__wrapper {
        border-radius: var(--radius-md);
        box-shadow: none;
        border: 2px solid rgba(102, 126, 234, 0.1);
        transition: all 0.3s ease;
        background: rgba(255, 255, 255, 0.8);
        
        &:hover {
            border-color: rgba(102, 126, 234, 0.3);
        }
        
        &.is-focus {
            border-color: var(--primary-color);
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
    }
}

html.dark :deep(.el-input) {
    .el-input__wrapper {
        background: rgba(30, 30, 40, 0.8);
        border-color: rgba(102, 126, 234, 0.2);
        
        &:hover {
            border-color: rgba(102, 126, 234, 0.4);
        }
    }
}

:deep(.el-switch) {
    &.auth-switch {
        --el-switch-on-color: var(--primary-color);
    }
}

.el-button+.el-button {
    margin-left: 0%;
}
</style>