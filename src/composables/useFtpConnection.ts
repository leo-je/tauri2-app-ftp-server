import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Command } from 'tauri-plugin-shellx-api';
import { platform } from '@tauri-apps/plugin-os';
import clipboard from 'tauri-plugin-clipboard-api';
import { ElMessage } from 'element-plus';

export function useFtpConnection(port: () => number) {
  const ips = ref(['127.0.0.1']);

  const getIps = async () => {
    const ipList = await invoke<string[]>('get_primary_ipv4');
    ips.value = ipList.length > 2 ? ipList.slice(0, 2) : ipList.length === 0 ? ['127.0.0.1'] : ipList;
  };

  const copy = async (ip: string) => {
    const address = `ftp://${ip}:${port()}`;
    await clipboard.writeText(address);
    ElMessage({ type: 'success', message: 'connection.copied' });
  };

  const openDir = async (dirPath: string) => {
    if (!dirPath) return;
    const osType = await platform();
    new Command(osType === 'windows' ? 'explorer' : 'open', [dirPath]).execute();
  };

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return { ips, getIps, copy, openDir, formatBytes };
}
