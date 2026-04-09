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
                    {{ isStart ? $t('control.stop') : $t('control.start') }}
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
                    <div class="terminal-shell">
                        <div class="terminal-toolbar" @click="showLogs = !showLogs">
                            <div class="terminal-toolbar-main">
                                <span class="terminal-title">{{ $t('log.title') }}</span>
                                <el-button
                                    size="small"
                                    class="terminal-clear-button"
                                    @click.stop="clearLogs"
                                    :disabled="logs.length === 0"
                                >
                                    {{ $t('log.clear') }}
                                </el-button>
                            </div>
                            <SvgIcon :name="showLogs ? 'chevronUp' : 'chevronDown'" :size="16" class="expand-icon" />
                        </div>
                        <transition name="expand" @after-enter="handleLogPanelEntered">
                            <div v-if="showLogs" class="terminal-body">
                                <div v-if="logs.length === 0" class="terminal-empty">
                                    <span class="terminal-prompt-label">ftp@server:~$</span>
                                    <span class="terminal-empty-text">{{ $t('log.empty') }}</span>
                                    <span class="terminal-cursor"></span>
                                </div>
                                <div
                                    v-else
                                    ref="logViewport"
                                    class="terminal-viewport"
                                    @scroll="handleLogScroll"
                                >
                                    <div class="terminal-list">
                                        <div
                                            v-for="log in logs"
                                            :key="log._id"
                                            :class="['terminal-line', `is-${log.operation}`]"
                                        >
                                            <span class="terminal-line-time">[{{ formatLogTime(log.time) }}]</span>
                                            <span :class="['terminal-line-command', log.operation]">
                                                {{ getOperationCommand(log.operation) }}
                                            </span>
                                            <span class="terminal-line-path" :title="getLogTarget(log)">
                                                {{ getLogTarget(log) }}
                                            </span>
                                            <span v-if="log.bytes" class="terminal-line-bytes">
                                                {{ formatBytes(log.bytes) }}
                                            </span>
                                            <span v-if="shouldShowUsername(log.username)" class="terminal-line-user">
                                                @{{ log.username }}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </transition>
                    </div>
                </div>
            </transition>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, onMounted, onUnmounted, ref, nextTick, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ElButton, ElMessage, ElInput } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { Command } from 'tauri-plugin-shellx-api';
import { platform } from '@tauri-apps/plugin-os';
import store, { runtimeState } from '../store';
import { info, warn, error, attachConsole } from '@tauri-apps/plugin-log';
import { SvgIcon } from '../components/icons';
import clipboard from "tauri-plugin-clipboard-api";
import { validatePath, validatePort } from '../utils/validation';
import { listen, emit } from '@tauri-apps/api/event';

const { t } = useI18n();

// 常量定义
const ANIMATION_DURATION = 600; // 首次加载动画持续时间(ms)
const MESSAGE_DURATION_DEFAULT = 5000; // 消息提示默认持续时间(ms)
const MAX_LOG_ENTRIES = 100; // 日志最大条目数
const SCROLL_BOTTOM_THRESHOLD = 24; // 日志滚动底部阈值(px)

const dirPath = ref('');
const ips = ref(['127.0.0.1']);
const port = ref(21);
const isStart = ref(false);
const isFirstLoad = ref(true);
const startTime = ref<Date | null>(null);
const runTime = ref('00:00:00');
let runTimer: ReturnType<typeof setInterval> | null = null;
const showLogs = ref(false);
const logs = ref<Array<FtpOperationLog & { _id: number }>>([]);
const logViewport = ref<HTMLElement | null>(null);
const followLogs = ref(true);
const isTogglingServer = ref(false);
let logIdCounter = 0;
let unlistenLog: (() => void) | null = null;
let unlistenGlobalStart: (() => void) | null = null;
let unlistenGlobalStop: (() => void) | null = null;

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
    warn(`Failed to update tray menu: ${e instanceof Error ? e.message : String(e)}`);
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

onMounted(async () => {
    // 首次加载后移除动画，避免切换 tabs 时重复触发
    nextTick(() => {
        setTimeout(() => {
            isFirstLoad.value = false;
        }, ANIMATION_DURATION);
    });

    if (!unlistenGlobalStart) {
        unlistenGlobalStart = await listen('global-start-ftp', async () => {
            console.log('收到全局启动事件');
            if (!isStart.value) {
                await startFtpServer();
            }
        });
    }

    if (!unlistenGlobalStop) {
        unlistenGlobalStop = await listen('global-stop-ftp', async () => {
            console.log('收到全局停止事件');
            if (isStart.value) {
                await stopFtpServer();
            }
        });
    }
})

onUnmounted(() => {
    if (runTimer) {
        clearInterval(runTimer);
        runTimer = null;
    }

    if (unlistenLog) {
        unlistenLog();
        unlistenLog = null;
    }

    if (unlistenGlobalStart) {
        unlistenGlobalStart();
        unlistenGlobalStart = null;
    }

    if (unlistenGlobalStop) {
        unlistenGlobalStop();
        unlistenGlobalStop = null;
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
  try {
    await new Command(osType === 'windows' ? 'explorer' : 'open', [dirPath.value]).execute();
  } catch (e) {
    error(e instanceof Error ? e.message : 'Failed to open directory');
    ElMessage({ type: "error", message: t('message.openDirFailed') });
  }
}

async function selectDir() {
    try {
        const selected = await open({ directory: true, multiple: false });
        if (selected) {
            dirPath.value = selected;
            await store.set('selected', selected);
        }
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : t('message.selectDirFailed');
    error(errorMsg);
    ElMessage({ type: "error", message: errorMsg });
  }
}

function startOrStopServer() {
    if (isTogglingServer.value) {
        return;
    }

    isStart.value ? stopFtpServer() : startFtpServer();
}

const showMessage = (type: 'success' | 'warning' | 'info' | 'error', message: string, duration?: number) => {
    ElMessage({
        type,
        message,
        duration,
        grouping: true,
    });
};

const notifyToggleFinished = async () => {
    await emit('ftp-toggle-finished', {
        isRunning: isStart.value,
    });
};

async function stopFtpServer() {
    if (isTogglingServer.value || !isStart.value) {
        return;
    }

    isTogglingServer.value = true;
    try {
        await invoke('stop_ftp_server', {});
        showMessage("success", t('message.serviceStopped'));
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
        showMessage("error", t('message.serviceStopFailed'));
    } finally {
        isTogglingServer.value = false;
        await notifyToggleFinished();
    }
}

const getIps = async () => {
  const ipList = await invoke<string[]>('get_primary_ipv4');
  console.log('当前IP:', ipList);
  // 处理IP列表：空列表使用本地回环，超过2个只保留前2个
  if (ipList.length === 0) {
    ips.value = ['127.0.0.1'];
  } else if (ipList.length > 2) {
    ips.value = ipList.slice(0, 2);
  } else {
    ips.value = ipList;
  }
};

async function startFtpServer() {
    if (isTogglingServer.value || isStart.value) {
        return;
    }

    isTogglingServer.value = true;
    try {
        await getIps()

        // 验证路径
        if (!dirPath.value) {
            showMessage("warning", t('message.selectPath'));
            return;
        }

        const pathValidation = validatePath(dirPath.value);
        if (!pathValidation.valid) {
            showMessage("error", pathValidation.error || t('message.pathInvalid'));
            return;
        }

        // 验证端口
        const portValidation = validatePort(port.value);
        if (!portValidation.valid) {
            showMessage("error", portValidation.error || t('message.portInvalid'));
            return;
        }

        // 显示端口警告
        if (portValidation.warningKey) {
            const warningMessage = portValidation.warningParams
                ? t(portValidation.warningKey, portValidation.warningParams)
                : t(portValidation.warningKey);
            showMessage("warning", warningMessage, MESSAGE_DURATION_DEFAULT);
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
        showMessage("success", t('message.serviceStarted'));
        isStart.value = true;
        runtimeState.isServerRunning.value = true;
        await updateTrayMenu(true);
        await invoke('set_server_running', { running: true });
        startRunTimer();
        await loadLogs();
        await setupLogListener();
    } catch (e) {
        showMessage("error", t('message.serviceStartFailed'));
    } finally {
        isTogglingServer.value = false;
        await notifyToggleFinished();
    }
}

const loadLogs = async () => {
    try {
        const result = await invoke<FtpOperationLog[]>('get_ftp_operation_logs');
        logs.value = result.map(log => ({ ...log, _id: logIdCounter++ }));
  } catch (e) {
    error(`Failed to load logs: ${e instanceof Error ? e.message : String(e)}`);
  }
};

const setupLogListener = async () => {
    if (unlistenLog) {
        unlistenLog();
    }
  unlistenLog = await listen<FtpOperationLog>('ftp-operation-log', (event) => {
    logs.value.push({ ...event.payload, _id: logIdCounter++ });
    if (logs.value.length > MAX_LOG_ENTRIES) {
      logs.value.shift();
    }
  });
};

const clearLogs = async () => {
    try {
        await invoke('clear_ftp_operation_logs');
        logs.value = [];
        followLogs.value = true;
  } catch (e) {
    error(`Failed to clear logs: ${e instanceof Error ? e.message : String(e)}`);
  }
};

const getOperationCommand = (operation: string): string => {
    const commandMap: Record<string, string> = {
        download: 'RETR',
        upload: 'STOR',
        delete: 'DELE',
        mkdir: 'MKD',
        rmdir: 'RMD',
        rename: 'RNTO',
    };
    return commandMap[operation] || operation.toUpperCase();
};

const getLogTarget = (log: FtpOperationLog): string => {
    if (log.operation === 'rename' && log.extra) {
        return `${log.extra} -> ${log.path}`;
    }

    return log.path;
};

const formatLogTime = (time: string): string => {
    const [, clock] = time.split(' ');
    return clock || time;
};

const shouldShowUsername = (username?: string): boolean => {
    if (!username) {
        return false;
    }

    return username.trim().toLowerCase() !== 'anonymous';
};

const scrollLogsToBottom = (behavior: ScrollBehavior = 'auto') => {
    if (!logViewport.value) {
        return;
    }

    logViewport.value.scrollTo({
        top: logViewport.value.scrollHeight,
        behavior,
    });
};

const ensureLogsAtBottom = async (behavior: ScrollBehavior = 'auto') => {
    await nextTick();
    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            scrollLogsToBottom(behavior);
        });
    });
};

const handleLogPanelEntered = () => {
    if (!followLogs.value) {
        return;
    }

    scrollLogsToBottom('auto');
};

const handleLogScroll = () => {
    if (!logViewport.value) {
        return;
    }

    const distanceToBottom =
        logViewport.value.scrollHeight -
        logViewport.value.scrollTop -
        logViewport.value.clientHeight;
    followLogs.value = distanceToBottom < SCROLL_BOTTOM_THRESHOLD;
};

const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

watch(() => logs.value.length, async (length, previousLength) => {
    if (!showLogs.value || !followLogs.value) {
        return;
    }

    await ensureLogsAtBottom(previousLength === 0 || length === 0 ? 'auto' : 'smooth');
});

watch(showLogs, async (visible) => {
    if (!visible) {
        return;
    }

    followLogs.value = true;
    await ensureLogsAtBottom('auto');
});

</script>

<style lang="scss" scoped>
.ftp-container {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding: 20px;
    overflow: auto;
    scrollbar-width: none;
    -ms-overflow-style: none;
}

.ftp-container::-webkit-scrollbar {
    display: none;
    width: 0;
    height: 0;
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
    padding: 0;
    overflow: hidden;
    background:
        radial-gradient(circle at top right, rgba(57, 153, 255, 0.14), transparent 30%),
        radial-gradient(circle at left center, rgba(65, 232, 196, 0.08), transparent 38%),
        linear-gradient(180deg, rgba(13, 18, 29, 0.98), rgba(5, 8, 14, 0.99));
    border: 1px solid rgba(129, 241, 226, 0.12);
    box-shadow:
        0 24px 48px rgba(0, 7, 18, 0.36),
        inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.terminal-shell {
    display: flex;
    flex-direction: column;
}

.terminal-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    background: linear-gradient(180deg, rgba(28, 35, 49, 0.96), rgba(15, 19, 28, 0.96));
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    cursor: pointer;
    user-select: none;
}

.terminal-toolbar-main {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
}

.terminal-title {
    font-size: 14px;
    font-weight: 600;
    color: #f8fafc;
}

.expand-icon {
    color: #94a3b8;
    flex-shrink: 0;
}

.terminal-body {
    padding: 16px;
}

.terminal-prompt-label {
    color: #7cf9d0;
    flex-shrink: 0;
}

.terminal-empty-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.terminal-clear-button {
    border: 1px solid rgba(125, 211, 252, 0.22);
    background: rgba(25, 35, 52, 0.92);
    color: #dbe7f3;

    &:not(.is-disabled):hover {
        border-color: rgba(125, 211, 252, 0.4);
        background: rgba(36, 50, 74, 0.96);
    }
}

.terminal-empty,
.terminal-viewport {
    border-radius: 14px;
    border: 1px solid rgba(125, 211, 252, 0.14);
    background:
        linear-gradient(rgba(255, 255, 255, 0.015), rgba(255, 255, 255, 0.015)),
        linear-gradient(180deg, rgba(6, 10, 17, 0.95), rgba(3, 6, 12, 0.98));
}

.terminal-empty {
    min-height: 180px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 20px;
    font-family: 'SFMono-Regular', 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    color: #94a3b8;
}

.terminal-cursor {
    width: 10px;
    height: 18px;
    border-radius: 2px;
    background: #61f3a6;
    animation: terminalBlink 1.1s steps(2, start) infinite;
}

.terminal-viewport {
  position: relative;
  max-height: 340px;
  overflow-y: auto;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.02);

  /* 自定义滚动条样式 */
  scrollbar-width: thin;
  scrollbar-color: rgba(125, 211, 252, 0.3) transparent;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(125, 211, 252, 0.3);
    border-radius: 3px;

    &:hover {
      background: rgba(125, 211, 252, 0.5);
    }
  }
}

.terminal-viewport::after {
    content: '';
    position: sticky;
    inset: 0;
    display: block;
    height: 0;
    pointer-events: none;
    box-shadow: inset 0 -24px 28px rgba(3, 6, 12, 0.32);
}

.terminal-list {
  padding: 4px 0;
  display: flex;
  flex-direction: column;
}

.terminal-line {
  display: flex;
  align-items: baseline;
  gap: 6px;
  padding: 2px 10px;
  font-size: 11px;
  line-height: 1.4;
  font-family: 'SFMono-Regular', 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  color: #d7e0ea;
  border-left: 2px solid transparent;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: background 0.15s ease, border-color 0.15s ease;

  &:hover {
    background: rgba(125, 211, 252, 0.04);
    border-left-color: rgba(125, 211, 252, 0.3);
  }
}

.terminal-line-time {
  color: #64748b;
  flex-shrink: 0;
  font-size: 10px;
}

.terminal-line-command {
  padding: 1px 6px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.04em;
  flex-shrink: 0;

  &.download {
    background: rgba(96, 165, 250, 0.12);
    color: #7dd3fc;
  }

  &.upload {
    background: rgba(74, 222, 128, 0.12);
    color: #86efac;
  }

  &.delete {
    background: rgba(248, 113, 113, 0.12);
    color: #fca5a5;
  }

  &.mkdir,
  &.rmdir {
    background: rgba(250, 204, 21, 0.12);
    color: #fde68a;
  }

  &.rename {
    background: rgba(196, 181, 253, 0.12);
    color: #c4b5fd;
  }
}

.terminal-line-path {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #f8fafc;
  font-size: 11px;
}

.terminal-line-bytes {
  color: #facc15;
  flex-shrink: 0;
  font-size: 10px;
}

.terminal-line-user {
  color: #c4b5fd;
  flex-shrink: 0;
  font-size: 10px;
}

@keyframes terminalBlink {
    50% {
        opacity: 0;
    }
}

@media (max-width: 640px) {
    .terminal-toolbar,
    .terminal-line {
        flex-wrap: wrap;
    }

    .terminal-toolbar-main {
        width: 100%;
    }

    .terminal-line-path {
        width: 100%;
        flex-basis: 100%;
    }

  .terminal-viewport {
    max-height: 520px;
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
