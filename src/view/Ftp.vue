<template>
    <div class="ftp-container">
        <div class="ftp-content">
            <!-- 标题区域 -->
            <div class="header-section fade-in" style="display: none;">
                <div class="icon-wrapper">
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2"/>
                        <path d="M9 12L11 14L15 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </div>
                <h1 class="ftp-title">FTP 服务器</h1>
                <p class="ftp-subtitle">快速启动您的文件传输服务</p>
            </div>

            <!-- 状态指示器 -->
            <div :class="['status-section', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.1s;' : ''">
                <div :class="['status-indicator', isStart ? 'running' : 'stopped']">
                    <span class="status-dot"></span>
                    {{ isStart ? '运行中' : '已停止' }}
                </div>
            </div>

            <!-- 配置卡片 -->
            <div :class="['config-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.2s;' : ''">
                <div class="card-header">
                    <svg class="card-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 15V17M6 21H18C19.1046 21 20 20.1046 20 19V13C20 11.8954 19.1046 11 18 11H6C4.89543 11 4 11.8954 4 13V19C4 20.1046 4.89543 21 6 21ZM16 11V7C16 4.79086 14.2091 3 12 3C9.79086 3 8 4.79086 8 7V11H16Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                    <span>服务器配置</span>
                </div>
                
                <div class="form-section">
                    <div class="form-item">
                        <label class="form-label">
                            <svg class="label-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M3 7V17C3 18.1046 3.89543 19 5 19H19C20.1046 19 21 18.1046 21 17V9C21 7.89543 20.1046 7 19 7H13L11 5H5C3.89543 5 3 5.89543 3 7Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                            共享目录
                        </label>
                        <div class="input-group">
                            <el-input 
                                v-model="dirPath" 
                                disabled 
                                placeholder="请选择共享目录"
                                class="ftp-input"
                            >
                                <template #prefix>
                                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" style="width: 16px; height: 16px;">
                                        <path d="M3 7V17C3 18.1046 3.89543 19 5 19H19C20.1046 19 21 18.1046 21 17V9C21 7.89543 20.1046 7 19 7H13L11 5H5C3.89543 5 3 5.89543 3 7Z" stroke="currentColor" stroke-width="2"/>
                                    </svg>
                                </template>
                            </el-input>
                            <el-button 
                                class="ftp-button ftp-button-primary"
                                :disabled="isStart" 
                                @click="selectDir"
                            >
                                选择
                            </el-button>
                            <el-button 
                                class="ftp-button"
                                @click="openDir"
                            >
                                打开
                            </el-button>
                        </div>
                    </div>

                    <div class="form-item">
                        <label class="form-label">
                            <svg class="label-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M8 9L11 12L8 15M13 9L16 12L13 15" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                <rect x="3" y="3" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            服务端口
                        </label>
                        <el-input 
                            :disabled="isStart" 
                            v-model="port" 
                            placeholder="21"
                            class="ftp-input"
                            type="number"
                        >
                            <template #prefix>
                                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" style="width: 16px; height: 16px;">
                                    <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2"/>
                                    <path d="M12 2V6M12 18V22M2 12H6M18 12H22" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                                </svg>
                            </template>
                        </el-input>
                    </div>
                </div>
            </div>

            <!-- 控制按钮 -->
            <div :class="['control-section', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.3s;' : ''">
                <el-button 
                    :type="isStart ? 'danger' : 'success'"
                    @click="startOrStopServer" 
                    class="control-button"
                    :class="{ 'is-running': isStart }"
                >
                    <svg class="button-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path v-if="!isStart" d="M8 5V19L19 12L8 5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path v-else d="M6 6H18M6 12H18M6 18H18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                    </svg>
                    {{ isStart ? '停止服务' : '启动服务' }}
                </el-button>
            </div>

            <!-- 连接信息 -->
            <transition name="slide-fade">
                <div v-if="isStart" :class="['connection-card', 'ftp-card', { 'fade-in': isFirstLoad }]" :style="isFirstLoad ? 'animation-delay: 0.4s;' : ''">
                    <div class="card-header">
                        <svg class="card-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M13.8284 10.1716C12.2663 8.60948 9.73367 8.60948 8.17157 10.1716L6.75736 11.5858M17.2426 11.5858L18.6569 10.1716C20.219 8.60948 20.219 6.07683 18.6569 4.51473C17.0948 2.95263 14.5621 2.95263 13 4.51473L11.5858 5.92894M10.1716 13.8284C11.7337 15.3905 14.2663 15.3905 15.8284 13.8284L17.2426 12.4142M6.75736 12.4142L5.34315 13.8284C3.78105 15.3905 3.78105 17.9232 5.34315 19.4853C6.90524 21.0474 9.4379 21.0474 11 19.4853L12.4142 18.0711" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                        <span>连接地址</span>
                    </div>
                    <div class="connection-links">
                        <div
                            v-for="ip in ips"
                            :key="ip"
                            class="connection-link"
                            @click="copy(ip)"
                        >
                            <svg class="link-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <rect x="8" y="8" width="12" height="12" rx="2" stroke="currentColor" stroke-width="2"/>
                                <path d="M16 8V6C16 4.89543 15.1046 4 14 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H8" stroke="currentColor" stroke-width="2"/>
                            </svg>
                            <span class="link-text">{{ 'ftp://' + ip + ':' + port }}</span>
                            <svg class="copy-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M8 5V19L19 12L8 5Z" fill="currentColor"/>
                            </svg>
                        </div>
                    </div>

                    <!-- 运行时间 -->
                    <div class="run-time-section">
                        <svg class="time-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
                            <path d="M12 6V12L16 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                        </svg>
                        <span class="run-time-label">运行时间</span>
                        <span class="run-time-value">{{ runTime }}</span>
                    </div>
                </div>
            </transition>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, onMounted, onUnmounted, ref, nextTick } from "vue";
import { ElButton, ElMessage, ElInput } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { Command } from 'tauri-plugin-shellx-api';
import { platform } from '@tauri-apps/plugin-os';
import store from '../store';
import { info, error, attachConsole } from '@tauri-apps/plugin-log';

const dirPath = ref('');
const ips = ref(['127.0.0.1']);
const port = ref(21);
const isStart = ref(false);
const isFirstLoad = ref(true);
const startTime = ref<Date | null>(null);
const runTime = ref('00:00:00');
let runTimer: ReturnType<typeof setInterval> | null = null;

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
})

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
import clipboard from "tauri-plugin-clipboard-api";

const copy = async (ip: string) => {
    // 拼装地址放入剪贴板
    let address = `ftp://${ip}:${port.value}`;
    await clipboard.writeText(address);
    ElMessage({ type: "success", message: '已复制到剪贴板' });
};

async function openDir() {
    if (!dirPath.value) {
        ElMessage({ type: "warning", message: "未选择目录" });
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
        const result = await invoke('stop_ftp_server', {}) || '';
        ElMessage({ type: "success", message: result.toString() });
        isStart.value = false;
        stopRunTimer();
    } catch (e) {
        ElMessage({ type: "error", message: e ? e.toString() : "未知错误" });
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
        if (!dirPath.value) {
            ElMessage("请选择路径");
            return;
        }
        logl("invoke-'start_ftp_server'");
        const users = (await store.get('tableData')) || [];
        const isAnonymous = await store.get('isAnonymous') || false;
        let fileauth = await store.get('fileauth') || 'R';

        const result = await invoke('start_ftp_server', {
            path: dirPath.value,
            port: port.value.toString(),
            users: JSON.stringify(users),
            isAnonymous,
            fileauth,
        }) || '';
        ElMessage({ type: "success", message: result.toString() });
        isStart.value = true;
        startRunTimer();
    } catch (e) {
        ElMessage({ type: "error", message: e ? e.toString() : "未知错误" });
    }
}

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