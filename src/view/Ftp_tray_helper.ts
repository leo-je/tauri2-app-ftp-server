// 辅助函数：更新后端托盘状态
async function updateTrayMenuStatus(running: boolean) {
  await emit('server-status-changed', { running });
}

async function stopFtpServer() {
  try {
    await invoke('stop_ftp_server', {});
    ElMessage({ type: "success", message: t('message.serviceStopped') });
    isStart.value = false;
    runtimeState.isServerRunning.value = false;
    stopRunTimer();
    // 更新托盘菜单
    await updateTrayMenuStatus(false);
  } catch (e) {
    ElMessage({ type: "error", message: t('message.serviceStopFailed') });
  }
}

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
    startRunTimer();
    // 更新托盘菜单
    await updateTrayMenuStatus(true);
  } catch (e) {
    ElMessage({ type: "error", message: t('message.serviceStartFailed') });
  }
}
