import { ref, nextTick, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export interface FtpOperationLog {
  time: string;
  operation: string;
  path: string;
  bytes?: number;
  username?: string;
  extra?: string;
}

export function useFtpLogs() {
  const logs = ref<FtpOperationLog[]>([]);
  const showLogs = ref(false);
  const followLogs = ref(true);
  const logViewport = ref<HTMLElement | null>(null);
  let unlistenLog: (() => void) | null = null;

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
      logs.value.push(event.payload);
      if (logs.value.length > 100) {
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
      console.error('Failed to clear logs:', e);
    }
  };

  const cleanup = () => {
    if (unlistenLog) {
      unlistenLog();
      unlistenLog = null;
    }
  };

  const scrollLogsToBottom = (behavior: ScrollBehavior = 'auto') => {
    if (!logViewport.value) return;
    logViewport.value.scrollTo({ top: logViewport.value.scrollHeight, behavior });
  };

  const ensureLogsAtBottom = async (behavior: ScrollBehavior = 'auto') => {
    await nextTick();
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        scrollLogsToBottom(behavior);
      });
    });
  };

  const handleLogScroll = () => {
    if (!logViewport.value) return;
    const distanceToBottom = logViewport.value.scrollHeight - logViewport.value.scrollTop - logViewport.value.clientHeight;
    followLogs.value = distanceToBottom < 24;
  };

  const handleLogPanelEntered = () => {
    if (!followLogs.value) return;
    scrollLogsToBottom('auto');
  };

  watch(() => logs.value.length, async (length, previousLength) => {
    if (!showLogs.value || !followLogs.value) return;
    await ensureLogsAtBottom(previousLength === 0 || length === 0 ? 'auto' : 'smooth');
  });

  watch(showLogs, async (visible) => {
    if (!visible) return;
    followLogs.value = true;
    await ensureLogsAtBottom('auto');
  });

  const getOperationCommand = (operation: string): string => {
    const commandMap: Record<string, string> = {
      download: 'RETR', upload: 'STOR', delete: 'DELE',
      mkdir: 'MKD', rmdir: 'RMD', rename: 'RNTO',
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
    return !!username && username.trim().toLowerCase() !== 'anonymous';
  };

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return {
    logs, showLogs, followLogs, logViewport,
    loadLogs, setupLogListener, clearLogs, cleanup,
    handleLogScroll, handleLogPanelEntered,
    getOperationCommand, getLogTarget, formatLogTime,
    shouldShowUsername, formatBytes,
  };
}
