<template>
    <div class="ftp-container">
        <div class="ftp-content">
            <!-- 标题区域 -->
            <div class="header-section fade-in" style="display: none;">
                <div class="icon-wrapper">
                    <SvgIcon name="checkCircle" :size="24" />
                </div>
                <h1 class="ftp-title">FTP 服务器</h1>
                <p class="ftp-subtitle">快速启动您的文件传输服务</p>
            </div>

            <!-- 状态指示器 -->
            <div :class="['status-section', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.1s;' : ''">
                <div :class="['status-indicator', isStart ? 'running' : 'stopped']">
                    <span class="status-dot"></span>
                    {{ isStart ? $t('status.running') : $t('status.stopped') }}
                </div>
            </div>

            <!-- 配置卡片 -->
            <div :class="['config-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.2s;' : ''">
                <div class="card-header">
                    <SvgIcon name="lock" :size="20" class="card-icon" />
                    <span>{{ $t('config.title') }}</span>
                </div>

                <div class="form-section">
                    <div class="form-item">
                        <label class="form-label">
                            <SvgIcon name="folder" :size="16" class="label-icon" />
                            {{ $t('config.shareDir') }}
                        </label>
                        <div class="input-group">
                            <el-input
                                v-model="dirPath"
                                disabled
                                :placeholder="$t('config.placeholder.dir')"
                                class="ftp-input"
                            >
                                <template #prefix>
                                    <SvgIcon name="folder" :size="16" />
                                </template>
                            </el-input>
                            <el-button
                                class="ftp-button ftp-button-primary"
                                :disabled="isStart"
                                @click="selectDir"
                            >
                                {{ $t('config.select') }}
                            </el-button>
                            <el-button
                                class="ftp-button"
                                @click="openDir"
                            >
                                {{ $t('config.open') }}
                            </el-button>
                        </div>
                    </div>

                    <div class="form-item">
                        <label class="form-label">
                            <SvgIcon name="terminal" :size="16" class="label-icon" />
                            {{ $t('config.port') }}
                        </label>
                        <el-input
                            :disabled="isStart"
                            v-model="port"
                            :placeholder="$t('config.placeholder.port')"
                            class="ftp-input"
                            type="number"
                        >
                            <template #prefix>
                                <SvgIcon name="target" :size="16" />
                            </template>
                        </el-input>
                    </div>
                </div>
            </div>

            <!-- 控制按钮 -->
            <div :class="['control-section', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.3s;' : ''" style="display: none;">
                <el-button
                    :type="isStart ? 'danger' : 'success'"
                    @click="startOrStopServer"
                    class="control-button"
                    :class="{ 'is-running': isStart }"
                >
                    <SvgIcon :name="isStart ? 'stop' : 'play'" :size="20" class="button-icon" />
                    {{ isStart ? '停止服务' : '启动服务' }}
                </el-button>
            </div>

            <!-- 悬浮按钮 -->
            <div
                :class="['floating-btn', { 'is-running': isStart }]"
                @click="startOrStopServer"
                :title="isStart ? $t('control.stopTooltip') : $t('control.startTooltip')"
            >
                <SvgIcon name="plane" :size="28" class="plane-icon" />
            </div>

            <!-- 连接信息 -->
            <transition name="slide-fade">
                <div v-if="isStart" :class="['connection-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.4s;' : ''">
                    <div class="card-header">
                        <SvgIcon name="link" :size="20" class="card-icon" />
                        <span>{{ $t('connection.title') }}</span>
                    </div>
                    <div class="connection-links">
                        <div
                            v-for="ip in ips"
                            :key="ip"
                            class="connection-link"
                            @click="copy(ip)"
                        >
                            <SvgIcon name="copyOutline" :size="20" class="link-icon" />
                            <span class="link-text">{{ 'ftp://' + ip + ':' + port }}</span>
                            <SvgIcon name="copy" :size="16" class="copy-icon" />
                        </div>
                    </div>

                    <!-- 运行时间 -->
                    <div class="run-time-section">
                        <SvgIcon name="clock" :size="18" class="time-icon" />
                        <span class="run-time-label">{{ $t('status.runtime') }}</span>
                        <span class="run-time-value">{{ runTime }}</span>
                    </div>
                </div>
            </transition>

            <!-- 操作日志面板 -->
            <transition name="slide-fade">
                <div v-if="isStart" :class="['log-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.5s;' : ''">
                    <div class="card-header" @click="showLogs = !showLogs">
                        <SvgIcon name="fileText" :size="20" class="card-icon" />
                        <span>{{ $t('log.title') }}</span>
                        <span class="log-count">({{ logs.length }})</span>
                        <SvgIcon :name="showLogs ? 'chevronUp' : 'chevronDown'" :size="16" class="expand-icon" />
                    </div>
                    <transition name="expand">
                        <div v-if="showLogs" class="log-content">
                            <div class="log-actions">
                                <el-button size="small" @click="clearLogs" :disabled="logs.length === 0">
                                    {{ $t('log.clear') }}
                                </el-button>
                            </div>
                            <div v-if="logs.length === 0" class="log-empty">
                                {{ $t('log.empty') }}
                            </div>
                            <div v-else class="log-list">
                                <div v-for="(log, index) in logs" :key="index" class="log-item">
                                    <span class="log-time">{{ log.time }}</span>
                                    <span :class="['log-operation', log.operation]">
                                        {{ getOperationLabel(log.operation) }}
                                    </span>
                                    <span class="log-path" :title="log.path">{{ log.path }}</span>
                                    <span v-if="log.bytes" class="log-bytes">{{ formatBytes(log.bytes) }}</span>
                                    <span v-if="log.username" class="log-user">{{ log.username }}</span>
                                </div>
                            </div>
                        </div>
                    </transition>
                </div>
            </transition>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, onMounted, onUnmounted, ref, nextTick } from "vue";
import { useI18n } from "vue-i18n";
import { ElButton, ElMessage, ElInput } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { Command } from 'tauri-plugin-shellx-api';
import { platform } from '@tauri-apps/plugin-os';
import store from '../store';
import { info, error, attachConsole } from '@tauri-apps/plugin-log';
import { SvgIcon } from '../components/icons';
import clipboard from "tauri-plugin-clipboard-api";
import { runtimeState } from '../store';
import { validatePath, validatePort } from '../utils/validation';
import { listen } from '@tauri-apps/api/event';

const { t } = useI18n();

const dirPath = ref('');
const ips = ref(['127.0.0.1']);
const port = ref(21);
const isStart = ref(false);
const isFirstLoad = ref(true);
const startTime = ref<Date | null>(null);
const runTime = ref('00:00:00');
let runTimer: ReturnType<typeof setInterval> | null = null;
const showLogs = ref(true);
const logs = ref<FtpOperationLog[]>([]);
let unlistenLog: (() => void) | null = null;

interface FtpOperationLog {
    time: string;
    operation: string;
    path: string;
    bytes?: number;
    username?: string;
    extra?: string;
}

async function updateTrayMenu(isRunning: boolean) {
  try {
    await invoke('update_tray_menu_language', {
      isRunning,
      text: {
        show: t('tray.show'),
        start: t('tray.start'),
        stop: t('tray.stop'),
        quit: t('tray.quit'),
        uptime: t('tray.uptime')
      }
    });
  } catch (e) {
    console.warn('Failed to update tray menu:', e);
  }
}

async function init() {
    const detach = await attachConsole();
    detach();
    const selected = await store.get('selected');
    if (selected) {
        info(selected.toString());
        dirPath.value = selected.toString();
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

  // 监听全局启动 FTP 事件
  listen('global-start-ftp', async () => {
    console.log('收到全局启动事件');
    if (!isStart.value) {
      await startFtpServer();
    }
  });

  // 监听全局停止 FTP 事件
  listen('global-stop-ftp', async () => {
    console.log('收到全局停止事件');
    if (isStart.value) {
      await stopFtpServer();
    }
  });})

onUnmounted(() => {
    if (runTimer) {
        clearInterval(runTimer);
        runTimer = null;
    }
})

const formatRunTime = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

const startRunTimer = () => {
    startTime.value = new Date();
    runTimer = setInterval(() => {
        if (startTime.value) {
            const elapsed = Math.floor((Date.now() - startTime.value.getTime()) / 1000);
            runTime.value = formatRunTime(elapsed);
        }
    }, 1000);
}

const stopRunTimer = () => {
    if (runTimer) {
        clearInterval(runTimer);
        runTimer = null;
    }
    startTime.value = null;
    runTime.value = '00:00:00';
}

const logl = (msg: string) => info(msg);

const checkPlatform = () => {
    const currentPlatform = platform();
    const platformMap: Record<string, string> = {
        windows: 'Windows',
        macos: 'macOS',
        linux: 'Linux',
        ios: 'iOS',
        android: 'Android',
    };
    const platformName = platformMap[currentPlatform] || `未知操作系统：${currentPlatform}`;
    logl(`当前操作系统是 ${platformName}`);
    return currentPlatform;
};

const copy = async (ip: string) => {
    // 拼装地址放入剪贴板
    let address = `ftp://${ip}:${port.value}`;
    await clipboard.writeText(address);
    ElMessage({ type: "success", message: t('connection.copied') });
};

async function openDir() {
    if (!dirPath.value) {
        ElMessage({ type: "warning", message: t('message.noDir') });
        return;
    }
    const osType = checkPlatform();
    new Command(osType === 'windows' ? 'explorer' : 'open', [dirPath.value]).execute();
}

async function selectDir() {
    try {
        const selected = await open({ directory: true, multiple: false });
        if (selected) {
            dirPath.value = selected;
            await store.set('selected', selected);
        }
    } catch (e) {
        error(e ? e.toString() : '读取目录失败');
        ElMessage(e ? e.toString() : '读取目录失败');
    }
}

function startOrStopServer() {
    isStart.value ? stopFtpServer() : startFtpServer();
}

async function stopFtpServer() {
    try {
        await invoke('stop_ftp_server', {});
        ElMessage({ type: "success", message: t('message.serviceStopped') });
        isStart.value = false;
        runtimeState.isServerRunning.value = false;
        await updateTrayMenu(false);
        stopRunTimer();
        if (unlistenLog) {
            unlistenLog();
            unlistenLog = null;
        }
    } catch (e) {
    // 更新后端状态
    await invoke('set_server_running', { running: false });
        ElMessage({ type: "error", message: t('message.serviceStopFailed') });
    }
}

const getIps = async () => {
    const ipList = await invoke<string[]>('get_primary_ipv4');
    console.log('当前IP:', ipList);
    // 判断数组长度,大于2则保留2
    if (ipList.length > 2) {
        ipList.splice(2, ipList.length - 2)
    } else if (ipList.length == 0) {
        ipList.push('127.0.0.1')
    }
    ips.value = ipList
};

async function startFtpServer() {
    try {
        await getIps()

        // 验证路径
        if (!dirPath.value) {
            ElMessage({ type: "warning", message: t('message.selectPath') });
            return;
        }

        const pathValidation = validatePath(dirPath.value);
        if (!pathValidation.valid) {
            ElMessage({ type: "error", message: pathValidation.error || t('message.pathInvalid') });
            return;
        }

        // 验证端口
        const portValidation = validatePort(port.value);
        if (!portValidation.valid) {
            ElMessage({ type: "error", message: portValidation.error || t('message.portInvalid') });
            return;
        }

        // 显示端口警告
        if (portValidation.warningKey) {
            const warningMessage = portValidation.warningParams
                ? t(portValidation.warningKey, portValidation.warningParams)
                : t(portValidation.warningKey);
            ElMessage({ type: "warning", message: warningMessage, duration: 5000 });
        }

        logl("invoke-'start_ftp_server'");
        const users = (await store.get('tableData')) || [];
        const isAnonymous = await store.get('isAnonymous') || false;
        let fileAuth = await store.get('fileauth') || 'R';

        await invoke('start_ftp_server', {
            path: dirPath.value,
            port: port.value.toString(),
            users: JSON.stringify(users),
            isAnonymous,
            fileAuth
        });
        ElMessage({ type: "success", message: t('message.serviceStarted') });
        isStart.value = true;
        runtimeState.isServerRunning.value = true;
        await updateTrayMenu(true);
        await invoke('set_server_running', { running: true });
        startRunTimer();
        await loadLogs();
        setupLogListener();
    } catch (e) {
        ElMessage({ type: "error", message: t('message.serviceStartFailed') });
    }
}

const loadLogs = async () => {
    try {
        const result = await invoke<FtpOperationLog[]>('get_ftp_operation_logs');
        logs.value = result;
    } catch (e) {
        console.error('Failed to load logs:', e);
    }
};

const setupLogListener = async () => {
    if (unlistenLog) {
        unlistenLog();
    }
    unlistenLog = await listen<FtpOperationLog>('ftp-operation-log', (event) => {
        logs.value.unshift(event.payload);
        if (logs.value.length > 100) {
            logs.value.pop();
        }
    });
};

const clearLogs = async () => {
    try {
        await invoke('clear_ftp_operation_logs');
        logs.value = [];
    } catch (e) {
        console.error('Failed to clear logs:', e);
    }
};

const getOperationLabel = (operation: string): string => {
    const key = `log.operations.${operation}`;
    return t(key) || operation;
};

const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

</script>

<style lang="scss" scoped>
.ftp-container {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding: 20px;
    overflow: auto;
}

.ftp-content {
    width: 100%;
    max-width: 600px;
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

.status-section {
    display: flex;
    justify-content: center;
    margin-bottom: 24px;
}

.config-card,
.connection-card {
    margin-bottom: 20px;
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

html.dark .card-header {
    color: #e0e0e0;
}

.form-section {
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.form-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.form-label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: 500;
    color: #606266;
    
    .label-icon {
        width: 16px;
        height: 16px;
        color: var(--primary-color);
    }
}

html.dark .form-label {
    color: #b0b0b0;
}

.input-group {
    display: flex;
    gap: 8px;
    
    .el-input {
        flex: 1;
    }
}

.control-section {
    display: flex;
    justify-content: center;
    margin: 32px 0;
}

/* 悬浮按钮 */
.floating-btn {
    position: fixed;
    right: 30px;
    bottom: 30px;
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: var(--gradient-success);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 4px 20px rgba(72, 198, 239, 0.4);
    transition: all 0.3s ease;
    z-index: 100;

    .plane-icon {
        width: 28px;
        height: 28px;
        transform: rotate(-45deg);
        transition: transform 0.3s ease;
    }

    &:hover {
        transform: scale(1.1);
        box-shadow: 0 6px 28px rgba(72, 198, 239, 0.5);

        .plane-icon {
            transform: rotate(-45deg) translateX(2px);
        }
    }

    &:active {
        transform: scale(0.95);
    }

    &.is-running {
        background: linear-gradient(135deg, #f56565 0%, #ed8936 100%);
        box-shadow: 0 4px 20px rgba(245, 101, 101, 0.4);

        .plane-icon {
            transform: rotate(135deg);
        }

        &:hover {
            box-shadow: 0 6px 28px rgba(245, 101, 101, 0.5);
        }
    }
}

html.dark .floating-btn {
    &:not(.is-running) {
        box-shadow: 0 4px 20px rgba(72, 198, 239, 0.3);
    }

    &.is-running {
        box-shadow: 0 4px 20px rgba(245, 101, 101, 0.3);
    }
}

.control-button {
    min-width: 160px;
    height: 48px;
    font-size: 16px;
    font-weight: 600;
    border-radius: var(--radius-lg);
    border: none;
    box-shadow: var(--shadow-lg);
    transition: all 0.3s ease;

    &:not(.is-running) {
        background: var(--gradient-success);
        color: white;

        &:hover {
            transform: translateY(-2px);
            box-shadow: 0 12px 24px rgba(72, 198, 239, 0.3);
        }
    }

    &.is-running {
        background: linear-gradient(135deg, #f56565 0%, #ed8936 100%);
        color: white;

        &:hover {
            transform: translateY(-2px);
            box-shadow: 0 12px 24px rgba(245, 101, 101, 0.3);
        }
    }

    .button-icon {
        width: 20px;
        height: 20px;
        margin-right: 8px;
    }
}

.connection-links {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.connection-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all 0.3s ease;
    border: 1px solid rgba(102, 126, 234, 0.1);
    
    &:hover {
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
        border-color: rgba(102, 126, 234, 0.3);
        transform: translateX(4px);
    }
    
    .link-icon {
        width: 20px;
        height: 20px;
        color: var(--primary-color);
        flex-shrink: 0;
    }
    
    .link-text {
        flex: 1;
        font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
        font-size: 14px;
        color: #303133;
        font-weight: 500;
    }
    
    .copy-icon {
        width: 16px;
        height: 16px;
        color: var(--primary-color);
        opacity: 0;
        transition: opacity 0.3s ease;
    }
    
    &:hover .copy-icon {
        opacity: 1;
    }
}

html.dark .connection-link {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
    border-color: rgba(102, 126, 234, 0.2);

    .link-text {
        color: #e0e0e0;
    }
}

/* 运行时间 */
.run-time-section {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid rgba(102, 126, 234, 0.1);

    .time-icon {
        width: 18px;
        height: 18px;
        color: var(--primary-color);
    }

    .run-time-label {
        font-size: 14px;
        color: #606266;
    }

    .run-time-value {
        font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
        font-size: 16px;
        font-weight: 600;
        color: var(--primary-color);
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
        padding: 4px 12px;
        border-radius: var(--radius-sm);
    }
}

html.dark .run-time-section {
    border-top-color: rgba(102, 126, 234, 0.2);

    .run-time-label {
        color: #b0b0b0;
    }
}

/* 日志面板 */
.log-card {
    margin-bottom: 20px;
    
    .card-header {
        cursor: pointer;
        user-select: none;
        
        .log-count {
            font-size: 12px;
            color: #909399;
            margin-left: 4px;
        }
        
        .expand-icon {
            margin-left: auto;
            transition: transform 0.3s;
        }
    }
}

.log-content {
    margin-top: 16px;
}

.log-actions {
    margin-bottom: 12px;
}

.log-empty {
    text-align: center;
    color: #909399;
    padding: 20px;
}

.log-list {
    max-height: 300px;
    overflow-y: auto;
    border: 1px solid rgba(102, 126, 234, 0.1);
    border-radius: var(--radius-md);
}

.log-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    font-size: 13px;
    border-bottom: 1px solid rgba(102, 126, 234, 0.05);
    
    &:last-child {
        border-bottom: none;
    }
    
    &:hover {
        background: rgba(102, 126, 234, 0.05);
    }
}

.log-time {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    color: #909399;
    flex-shrink: 0;
}

.log-operation {
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 500;
    flex-shrink: 0;
    
    &.download {
        background: rgba(64, 158, 255, 0.1);
        color: #409eff;
    }
    
    &.upload {
        background: rgba(103, 194, 58, 0.1);
        color: #67c23a;
    }
    
    &.delete {
        background: rgba(245, 108, 108, 0.1);
        color: #f56c6c;
    }
    
    &.mkdir, &.rmdir {
        background: rgba(230, 162, 60, 0.1);
        color: #e6a23c;
    }
    
    &.rename {
        background: rgba(144, 147, 153, 0.1);
        color: #909399;
    }
}

.log-path {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.log-bytes {
    color: #909399;
    font-size: 12px;
    flex-shrink: 0;
}

.log-user {
    color: #c0c4cc;
    font-size: 12px;
    flex-shrink: 0;
}

html.dark {
    .log-card {
        .card-header .log-count {
            color: #606266;
        }
    }
    
    .log-list {
        border-color: rgba(102, 126, 234, 0.2);
    }
    
    .log-item {
        border-bottom-color: rgba(102, 126, 234, 0.1);
        
        &:hover {
            background: rgba(102, 126, 234, 0.1);
        }
    }
    
    .log-operation {
        &.download {
            background: rgba(64, 158, 255, 0.2);
        }
        
        &.upload {
            background: rgba(103, 194, 58, 0.2);
        }
        
        &.delete {
            background: rgba(245, 108, 108, 0.2);
        }
        
        &.mkdir, &.rmdir {
            background: rgba(230, 162, 60, 0.2);
        }
        
        &.rename {
            background: rgba(144, 147, 153, 0.2);
        }
    }
}

/* 展开动画 */
.expand-enter-active,
.expand-leave-active {
    transition: all 0.3s ease;
    overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
    opacity: 0;
    max-height: 0;
}

.expand-enter-to,
.expand-leave-from {
    max-height: 500px;
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
    
    &.is-disabled .el-input__wrapper {
        background: rgba(0, 0, 0, 0.02);
        border-color: rgba(0, 0, 0, 0.05);
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
    
    &.is-disabled .el-input__wrapper {
        background: rgba(0, 0, 0, 0.2);
    }
}

:deep(.el-button) {
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: all 0.3s ease;
    
    &:not(.is-disabled):hover {
        transform: translateY(-1px);
    }
    
    &:active {
        transform: translateY(0);
    }
}
.el-button+.el-button{
    margin-left: 0px;
}
</style>