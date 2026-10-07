<template>
  <div class="server-page">
    <!-- ── 控制台：状态 / 运行时长 / 访问地址 / 启停 ───────────────── -->
    <section class="card deck" :class="{ 'is-running': isStart }">
      <div class="deck-head">
        <span class="led" :class="isStart ? 'is-live' : 'is-idle'"></span>
        <span class="deck-state">{{ isStart ? $t('status.running') : $t('status.stopped') }}</span>
        <span v-if="isStart" class="chip mono deck-port">:{{ port }}</span>
      </div>

      <div class="deck-readout">
        <div class="readout-value mono" :class="{ 'is-idle': !isStart }">{{ runTime }}</div>
        <div class="readout-label">{{ $t('status.runtime') }}</div>
      </div>

      <transition name="reveal">
        <div v-if="isStart" class="deck-addresses">
          <div class="section-label">{{ $t('connection.title') }}</div>
          <button
            v-for="ip in ips"
            :key="ip"
            type="button"
            class="address-row"
            :title="$t('connection.copyHint')"
            @click="copy(ip)"
          >
            <span class="address-text mono selectable">{{ 'ftp://' + ip + ':' + port }}</span>
            <SvgIcon
              :name="copiedIp === ip ? 'checkCircle' : 'copy'"
              :size="14"
              class="address-copy"
              :class="{ 'is-copied': copiedIp === ip }"
            />
          </button>
        </div>
      </transition>

      <el-button
        class="deck-action"
        :type="isStart ? 'danger' : 'primary'"
        :plain="isStart"
        :loading="isTogglingServer"
        @click="startOrStopServer"
      >
        <SvgIcon
          v-if="!isTogglingServer"
          :name="isStart ? 'stop' : 'play'"
          :size="14"
          class="deck-action-icon"
        />
        {{ isStart ? $t('control.stop') : $t('control.start') }}
      </el-button>
    </section>

    <!-- ── 服务器配置 ─────────────────────────────────────────────── -->
    <section class="card">
      <div class="card-head">
        <SvgIcon name="settings" :size="16" class="card-head-icon" />
        <span class="card-title">{{ $t('config.title') }}</span>
        <span v-if="isStart" class="chip card-head-actions">{{ $t('config.locked') }}</span>
      </div>

      <div class="card-body config-body">
        <div class="field">
          <label class="field-label">{{ $t('config.shareDir') }}</label>
          <el-input
            v-model="dirPath"
            disabled
            class="is-mono selectable"
            :placeholder="$t('config.placeholder.dir')"
          >
            <template #prefix>
              <SvgIcon name="folder" :size="14" />
            </template>
          </el-input>
        </div>

        <div class="field-actions">
          <el-button :disabled="isStart" @click="selectDir">{{ $t('config.select') }}</el-button>
          <el-button @click="openDir">{{ $t('config.open') }}</el-button>
        </div>

        <div class="field">
          <label class="field-label" for="ftp-port">{{ $t('config.port') }}</label>
          <el-input
            id="ftp-port"
            v-model="port"
            class="is-mono port-input"
            type="number"
            :disabled="isStart"
            :placeholder="$t('config.placeholder.port')"
          />
        </div>
      </div>
    </section>

    <!-- ── 操作日志 ──────────────────────────────────────────────── -->
    <section v-if="isStart || logs.length > 0" class="card console-card">
      <div class="console-head">
        <button
          type="button"
          class="console-toggle"
          :aria-expanded="showLogs"
          @click="showLogs = !showLogs"
        >
          <SvgIcon name="terminal" :size="15" class="card-head-icon" />
          <span class="card-title">{{ $t('log.title') }}</span>
          <span class="chip mono console-count">{{ logs.length }}</span>
          <SvgIcon
            :name="showLogs ? 'chevronUp' : 'chevronDown'"
            :size="14"
            class="console-caret"
          />
        </button>
        <button
          type="button"
          class="icon-btn"
          :disabled="logs.length === 0"
          :title="$t('log.clear')"
          :aria-label="$t('log.clear')"
          @click="clearLogs"
        >
          <SvgIcon name="delete" :size="14" />
        </button>
      </div>

      <transition name="reveal" @after-enter="handleLogPanelEntered">
        <div v-if="showLogs" class="console-body">
          <div v-if="logs.length === 0" class="console-empty">
            <span class="console-prompt mono">ftp@server:~$</span>
            <span class="console-empty-text">{{ $t('log.empty') }}</span>
            <span class="console-cursor"></span>
          </div>
          <div
            v-else
            ref="logViewport"
            class="console-viewport"
            @scroll="handleLogScroll"
          >
            <div class="console-lines">
              <div v-for="log in logs" :key="log._id" class="console-line">
                <span class="console-time mono">[{{ formatLogTime(log.time) }}]</span>
                <span class="console-op mono" :class="`is-${log.operation}`">
                  {{ getOperationCommand(log.operation) }}
                </span>
                <span class="console-path mono" :title="getLogTarget(log)">
                  {{ getLogTarget(log) }}
                </span>
                <span v-if="log.bytes" class="console-bytes mono">{{ formatBytes(log.bytes) }}</span>
                <span v-if="shouldShowUsername(log.username)" class="console-user mono">
                  @{{ log.username }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </transition>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onBeforeMount, onMounted, onUnmounted, ref, nextTick, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElButton, ElInput, ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Command } from 'tauri-plugin-shellx-api';
import { platform } from '@tauri-apps/plugin-os';
import store, { runtimeState } from '../store';
import { info, warn, error } from '@tauri-apps/plugin-log';
import { SvgIcon } from '../components/icons';
import clipboard from 'tauri-plugin-clipboard-api';
import { validatePath, validatePort } from '../utils/validation';
import { listen, emit } from '@tauri-apps/api/event';

const { t } = useI18n();

const MESSAGE_DURATION_DEFAULT = 5000; // 消息提示默认持续时间(ms)
const MAX_LOG_ENTRIES = 100; // 日志最大条目数
const SCROLL_BOTTOM_THRESHOLD = 24; // 日志滚动底部阈值(px)
const SCROLL_SETTLE_DURATION = 420; // 平滑滚动大致时长(ms)
const COPY_FEEDBACK_DURATION = 1400; // 复制成功图标的回执时长(ms)

const dirPath = ref('');
const ips = ref(['127.0.0.1']);
const port = ref(21);
const isStart = ref(false);
const startTime = ref<Date | null>(null);
const runTime = ref('00:00:00');
const copiedIp = ref('');
const showLogs = ref(true);
const logs = ref<Array<FtpOperationLog & { _id: number }>>([]);
const logViewport = ref<HTMLElement | null>(null);
const followLogs = ref(true);
const isTogglingServer = ref(false);

let runTimer: ReturnType<typeof setInterval> | null = null;
let copyFeedbackTimer: ReturnType<typeof setTimeout> | null = null;
let scrollSettleTimer: ReturnType<typeof setTimeout> | null = null;
let programmaticScroll = false;
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
  const selected = await store.get('selected');
  if (selected) {
    info(selected.toString());
    dirPath.value = selected.toString();
  }
}

onBeforeMount(() => {
  init();
});

onMounted(async () => {
  // 控制台默认展开，挂载后直接落到底部
  await ensureLogsAtBottom('auto');

  unlistenGlobalStart = await listen('global-start-ftp', async () => {
    if (!isStart.value) {
      await startFtpServer();
    }
  });

  unlistenGlobalStop = await listen('global-stop-ftp', async () => {
    if (isStart.value) {
      await stopFtpServer();
    }
  });
});

onUnmounted(() => {
  if (runTimer) {
    clearInterval(runTimer);
    runTimer = null;
  }

  if (copyFeedbackTimer) {
    clearTimeout(copyFeedbackTimer);
    copyFeedbackTimer = null;
  }

  if (scrollSettleTimer) {
    clearTimeout(scrollSettleTimer);
    scrollSettleTimer = null;
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
});

const formatRunTime = (seconds: number): string => {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs
    .toString()
    .padStart(2, '0')}`;
};

const startRunTimer = () => {
  startTime.value = new Date();
  runTimer = setInterval(() => {
    if (startTime.value) {
      const elapsed = Math.floor((Date.now() - startTime.value.getTime()) / 1000);
      runTime.value = formatRunTime(elapsed);
    }
  }, 1000);
};

const stopRunTimer = () => {
  if (runTimer) {
    clearInterval(runTimer);
    runTimer = null;
  }
  startTime.value = null;
  runTime.value = '00:00:00';
};

const showCopyFeedback = (ip: string) => {
  copiedIp.value = ip;
  if (copyFeedbackTimer) {
    clearTimeout(copyFeedbackTimer);
  }
  copyFeedbackTimer = setTimeout(() => {
    copiedIp.value = '';
  }, COPY_FEEDBACK_DURATION);
};

const copy = async (ip: string) => {
  try {
    await clipboard.writeText(`ftp://${ip}:${port.value}`);
  } catch (e) {
    error(`Failed to copy address: ${e instanceof Error ? e.message : String(e)}`);
    ElMessage({ type: 'error', message: t('connection.copyFailed'), grouping: true });
    return;
  }
  showCopyFeedback(ip);
  ElMessage({ type: 'success', message: t('connection.copied'), grouping: true });
};

async function openDir() {
  if (!dirPath.value) {
    ElMessage({ type: 'warning', message: t('message.noDir') });
    return;
  }
  const osType = platform();
  try {
    await new Command(osType === 'windows' ? 'explorer' : 'open', [dirPath.value]).execute();
  } catch (e) {
    error(e instanceof Error ? e.message : 'Failed to open directory');
    ElMessage({ type: 'error', message: t('message.openDirFailed') });
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
    ElMessage({ type: 'error', message: errorMsg });
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
    grouping: true
  });
};

const notifyToggleFinished = async () => {
  await emit('ftp-toggle-finished', {
    isRunning: isStart.value
  });
};

async function stopFtpServer() {
  if (isTogglingServer.value || !isStart.value) {
    return;
  }

  isTogglingServer.value = true;
  try {
    await invoke('stop_ftp_server', {});
    showMessage('success', t('message.serviceStopped'));
    isStart.value = false;
    runtimeState.isServerRunning.value = false;
    await updateTrayMenu(false);
    await invoke('set_server_running', { running: false });
    stopRunTimer();
    if (unlistenLog) {
      unlistenLog();
      unlistenLog = null;
    }
  } catch (e) {
    showMessage('error', t('message.serviceStopFailed'));
  } finally {
    isTogglingServer.value = false;
    await notifyToggleFinished();
  }
}

const getIps = async () => {
  const ipList = await invoke<string[]>('get_primary_ipv4');
  // 空列表退回回环地址，超过 2 个只保留前 2 个
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
    await getIps();

    if (!dirPath.value) {
      showMessage('warning', t('message.selectPath'));
      return;
    }

    const pathValidation = validatePath(dirPath.value);
    if (!pathValidation.valid) {
      showMessage('error', pathValidation.error || t('message.pathInvalid'));
      return;
    }

    const portValidation = validatePort(port.value);
    if (!portValidation.valid) {
      showMessage('error', portValidation.error || t('message.portInvalid'));
      return;
    }

    if (portValidation.warningKey) {
      const warningMessage = portValidation.warningParams
        ? t(portValidation.warningKey, portValidation.warningParams)
        : t(portValidation.warningKey);
      showMessage('warning', warningMessage, MESSAGE_DURATION_DEFAULT);
    }

    const users = (await store.get('tableData')) || [];
    const isAnonymous = (await store.get('isAnonymous')) || false;
    const fileAuth = (await store.get('fileauth')) || 'R';

    await invoke('start_ftp_server', {
      path: dirPath.value,
      port: port.value.toString(),
      users: JSON.stringify(users),
      isAnonymous,
      fileAuth
    });

    showMessage('success', t('message.serviceStarted'));
    isStart.value = true;
    runtimeState.isServerRunning.value = true;
    await updateTrayMenu(true);
    await invoke('set_server_running', { running: true });
    startRunTimer();
    await loadLogs();
    await setupLogListener();
    followLogs.value = true;
    await ensureLogsAtBottom('auto');
  } catch (e) {
    showMessage('error', t('message.serviceStartFailed'));
  } finally {
    isTogglingServer.value = false;
    await notifyToggleFinished();
  }
}

const loadLogs = async () => {
  try {
    const result = await invoke<FtpOperationLog[]>('get_ftp_operation_logs');
    logs.value = result.map((log) => ({ ...log, _id: logIdCounter++ }));
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
    list: 'LIST',
    cwd: 'CWD'
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

  // 平滑滚动会持续派发 scroll 事件，中途离底部还很远，
  // 会被 handleLogScroll 误判成「用户滚上去了」而关掉跟随。
  programmaticScroll = true;
  if (scrollSettleTimer) {
    clearTimeout(scrollSettleTimer);
  }
  scrollSettleTimer = setTimeout(
    () => {
      programmaticScroll = false;
    },
    behavior === 'smooth' ? SCROLL_SETTLE_DURATION : 0
  );

  logViewport.value.scrollTo({
    top: logViewport.value.scrollHeight,
    behavior
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
  if (!logViewport.value || programmaticScroll) {
    return;
  }

  const distanceToBottom =
    logViewport.value.scrollHeight - logViewport.value.scrollTop - logViewport.value.clientHeight;
  followLogs.value = distanceToBottom < SCROLL_BOTTOM_THRESHOLD;
};

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

watch(
  () => logs.value.length,
  async (length, previousLength) => {
    if (!showLogs.value || !followLogs.value) {
      return;
    }

    await ensureLogsAtBottom(previousLength === 0 || length === 0 ? 'auto' : 'smooth');
  }
);

watch(showLogs, async (visible) => {
  if (!visible) {
    return;
  }

  followLogs.value = true;
  await ensureLogsAtBottom('auto');
});
</script>

<style scoped lang="scss">
.server-page {
  display: flex;
  flex-direction: column;
  gap: var(--s-4);
}

/* ── 控制台 ─────────────────────────────────────────────────── */
.deck {
  position: relative;
  overflow: hidden;
  padding: var(--s-5);
  display: flex;
  flex-direction: column;
  gap: var(--s-4);
}

/* 顶部一条状态线：停止时是中性色，运行时亮起 */
.deck::before {
  content: '';
  position: absolute;
  inset: 0 0 auto 0;
  height: 2px;
  background: var(--border);
  transition: background var(--dur-3) var(--ease-standard);
}

.deck.is-running::before {
  background: linear-gradient(90deg, var(--success) 0%, var(--success-line) 45%, transparent 100%);
}

.deck-head {
  display: flex;
  align-items: center;
  gap: var(--s-2);
}

.deck-state {
  font-size: var(--fs-base);
  font-weight: 600;
  letter-spacing: -0.005em;
}

.deck-port {
  background: var(--surface-3);
  margin-left: 2px;
}

.readout-value {
  font-size: var(--fs-display);
  font-weight: 500;
  line-height: 1.05;
  letter-spacing: -0.025em;
  color: var(--text);
  transition: color var(--dur-2) var(--ease-standard);

  &.is-idle {
    color: var(--text-3);
  }
}

.readout-label {
  margin-top: 3px;
  font-size: var(--fs-2xs);
  font-weight: 600;
  letter-spacing: var(--tracking-wider);
  text-transform: uppercase;
  color: var(--text-3);
}

.deck-addresses {
  display: flex;
  flex-direction: column;
  gap: var(--s-2);
  padding-top: var(--s-4);
  border-top: 1px solid var(--border);
}

.address-row {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  width: 100%;
  height: 36px;
  padding: 0 var(--s-3);
  border: 1px solid var(--border);
  border-radius: var(--r-md);
  background: var(--surface-2);
  color: var(--text);
  cursor: pointer;
  text-align: left;
  transition: border-color var(--dur-1) var(--ease-standard),
    background var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--surface-3);
    border-color: var(--accent-line);
  }

  &:hover .address-copy {
    opacity: 1;
  }
}

.address-text {
  flex: 1;
  min-width: 0;
  font-size: var(--fs-md);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.address-copy {
  flex-shrink: 0;
  color: var(--text-3);
  opacity: 0.45;
  transition: opacity var(--dur-1) var(--ease-standard),
    color var(--dur-1) var(--ease-standard);

  &.is-copied {
    color: var(--success);
    opacity: 1;
  }
}

.deck-action {
  width: 100%;
  height: 40px;
  border-radius: var(--r-md);
  font-size: var(--fs-base);
  font-weight: 600;
}

.deck-action-icon {
  margin-right: 6px;
}

/* ── 配置 ──────────────────────────────────────────────────── */
.config-body {
  display: flex;
  flex-direction: column;
  gap: var(--s-3);
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
}

.field-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--s-2);
}

.port-input {
  width: 132px;
}

/* ── 日志控制台 ─────────────────────────────────────────────── */
.console-card {
  overflow: hidden;
}

.console-head {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  padding-right: var(--s-3);
  border-bottom: 1px solid var(--border);
}

.console-toggle {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  flex: 1;
  min-width: 0;
  height: 50px;
  padding: 0 var(--s-3) 0 var(--s-5);
  border: 0;
  background: transparent;
  cursor: pointer;
  color: inherit;
  text-align: left;

  &:hover .console-caret {
    color: var(--text);
  }
}

.console-count {
  background: var(--surface-3);
}

.console-caret {
  margin-left: auto;
  color: var(--text-3);
  transition: color var(--dur-1) var(--ease-standard);
}

.console-body {
  padding: var(--s-3);
}

.console-viewport,
.console-empty {
  border: 1px solid var(--console-border);
  border-radius: var(--r-md);
  background: var(--console-bg);
}

.console-viewport {
  max-height: 300px;
  overflow-y: auto;
}

.console-lines {
  padding: var(--s-1) 0;
}

.console-line {
  display: flex;
  align-items: baseline;
  gap: 7px;
  padding: 2px var(--s-3);
  font-size: var(--fs-xs);
  line-height: 1.45;
  color: var(--console-text);
  border-left: 2px solid transparent;
  white-space: nowrap;
  overflow: hidden;
  transition: background var(--dur-1) var(--ease-standard),
    border-color var(--dur-1) var(--ease-standard);

  &:hover {
    background: var(--console-hover);
    border-left-color: var(--accent-line);
  }
}

.console-time {
  flex-shrink: 0;
  font-size: var(--fs-2xs);
  color: var(--console-text-dim);
}

/* 操作码：定宽小徽标，方便竖着扫一列 */
.console-op {
  flex-shrink: 0;
  width: 40px;
  padding: 1px 0;
  border-radius: var(--r-xs);
  font-size: var(--fs-2xs);
  font-weight: 600;
  letter-spacing: 0.04em;
  text-align: center;

  &.is-download {
    background: var(--info-soft);
    color: var(--info);
  }

  &.is-upload {
    background: var(--success-soft);
    color: var(--success);
  }

  &.is-delete {
    background: var(--danger-soft);
    color: var(--danger);
  }

  &.is-mkdir,
  &.is-rmdir,
  &.is-rename {
    background: var(--accent-soft);
    color: var(--accent);
  }

  &.is-list,
  &.is-cwd {
    background: var(--muted-soft);
    color: var(--console-text-dim);
  }
}

.console-path {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.console-bytes {
  flex-shrink: 0;
  font-size: var(--fs-2xs);
  color: var(--accent);
}

.console-user {
  flex-shrink: 0;
  font-size: var(--fs-2xs);
  color: var(--console-text-dim);
}

.console-empty {
  display: flex;
  align-items: center;
  gap: var(--s-2);
  min-height: 88px;
  padding: var(--s-4);
  font-size: var(--fs-xs);
  color: var(--console-text-dim);
}

.console-prompt {
  color: var(--success);
  flex-shrink: 0;
}

.console-empty-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.console-cursor {
  width: 7px;
  height: 13px;
  border-radius: 1px;
  background: var(--success);
  animation: blink 1.1s steps(2, start) infinite;
}

/* ── 过渡 ──────────────────────────────────────────────────── */
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

@media (max-width: 560px) {
  .readout-value {
    font-size: var(--fs-2xl);
  }
}
</style>
