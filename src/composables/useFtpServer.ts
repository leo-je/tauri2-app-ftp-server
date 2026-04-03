import { ref, type Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, emit } from '@tauri-apps/api/event';
import { ElMessage } from 'element-plus';
import { info, attachConsole } from '@tauri-apps/plugin-log';
import store, { runtimeState } from '../store';
import { validatePath, validatePort } from '../utils/validation';

export function useFtpServer(
  dirPath: Ref<string>,
  port: Ref<number>,
  runTimer: { start: () => void; stop: () => void },
) {
  const isStart = ref(false);
  const isTogglingServer = ref(false);
  let unlistenGlobalStart: (() => void) | null = null;
  let unlistenGlobalStop: (() => void) | null = null;

  const init = async () => {
    const detach = await attachConsole();
    detach();
    const selected = await store.get('selected');
    if (selected) {
      info(selected.toString());
      dirPath.value = selected.toString();
    }
  };

  const updateTrayMenu = async (isRunning: boolean, t: (key: string) => string) => {
    try {
      await invoke('update_tray_menu_language', {
        isRunning,
        text: {
          show: t('tray.show'),
          start: t('tray.start'),
          stop: t('tray.stop'),
          quit: t('tray.quit'),
          uptime: t('tray.uptime'),
        },
      });
    } catch (e) {
      console.warn('Failed to update tray menu:', e);
    }
  };

  const showMessage = (type: 'success' | 'warning' | 'info' | 'error', message: string, duration?: number) => {
    ElMessage({ type, message, duration, grouping: true });
  };

  const notifyToggleFinished = async () => {
    await emit('ftp-toggle-finished', { isRunning: isStart.value });
  };

  const stopFtpServer = async (t: (key: string) => string) => {
    if (isTogglingServer.value || !isStart.value) return;

    isTogglingServer.value = true;
    try {
      await invoke('stop_ftp_server', {});
      showMessage('success', t('message.serviceStopped'));
      isStart.value = false;
      runtimeState.isServerRunning.value = false;
      await updateTrayMenu(false, t);
      runTimer.stop();
    } catch (e) {
      await invoke('set_server_running', { running: false });
      showMessage('error', t('message.serviceStopFailed'));
    } finally {
      isTogglingServer.value = false;
      await notifyToggleFinished();
    }
  };

  const startFtpServer = async (
    t: (key: string) => string,
    loadLogs: () => Promise<void>,
    setupLogListener: () => Promise<void>,
    getIps: () => Promise<void>,
  ) => {
    if (isTogglingServer.value || isStart.value) return;

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
        const warningMessage = t(portValidation.warningKey);
        showMessage('warning', warningMessage, 5000);
      }

      info("invoke-'start_ftp_server'");
      const users = (await store.get('tableData')) || [];
      const isAnonymous = await store.get('isAnonymous') || false;
      const fileAuth = await store.get('fileauth') || 'R';

      await invoke('start_ftp_server', {
        path: dirPath.value,
        port: port.value.toString(),
        users: JSON.stringify(users),
        isAnonymous,
        fileAuth,
      });
      showMessage('success', t('message.serviceStarted'));
      isStart.value = true;
      runtimeState.isServerRunning.value = true;
      await updateTrayMenu(true, t);
      await invoke('set_server_running', { running: true });
      runTimer.start();
      await loadLogs();
      await setupLogListener();
    } catch (e) {
      showMessage('error', t('message.serviceStartFailed'));
    } finally {
      isTogglingServer.value = false;
      await notifyToggleFinished();
    }
  };

  const setupGlobalListeners = async () => {
    if (!unlistenGlobalStart) {
      unlistenGlobalStart = await listen('global-start-ftp', async () => {
        if (!isStart.value) {
          await startFtpServer((key: string) => key, () => Promise.resolve(), () => Promise.resolve(), () => Promise.resolve());
        }
      });
    }
    if (!unlistenGlobalStop) {
      unlistenGlobalStop = await listen('global-stop-ftp', async () => {
        if (isStart.value) {
          await stopFtpServer((key: string) => key);
        }
      });
    }
  };

  const cleanupGlobalListeners = () => {
    if (unlistenGlobalStart) {
      unlistenGlobalStart();
      unlistenGlobalStart = null;
    }
    if (unlistenGlobalStop) {
      unlistenGlobalStop();
      unlistenGlobalStop = null;
    }
  };

  return {
    isStart, isTogglingServer, init,
    startFtpServer, stopFtpServer,
    setupGlobalListeners, cleanupGlobalListeners,
  };
}
