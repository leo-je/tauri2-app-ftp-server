<template>
    <div style="height: 100%; border: 1px;">
        <div style="width: 90%;margin: auto;text-align: left;">
            <!-- <div style="margin-top: 10px;height: 20px;"><span> </span></div> -->
            <div style="margin-top: 10px;">
                <div style="margin: 0px;"><span>目录：</span></div>
                <div>
                    <el-input v-model="dirPath" disabled placeholder="" style="width: 65%" />
                    <el-button style="margin-left: 5px;" :disabled="isStart" @click="selectDir">选择</el-button>
                    <el-button style="margin-left: 5px;" @click="openDir">打开</el-button>
                </div>
            </div>

            <div style="margin-top: 10px;">
                <div style="margin: 0px;"><span>端口：</span></div>
                <div>
                    <el-input :disabled="isStart" v-model="port" placeholder="21" />
                </div>
            </div>
            <div style="margin-top: 30px;margin-bottom: 10px;text-align: center;">
                <el-button type="success" @click="startOrStopServer" style="width: 100px;">
                    {{ isStart ? '停止' : '开始' }}
                </el-button>
            </div>

            <div style="margin-top: 20px;margin-bottom: 10px;">
                <template v-if="isStart">
                    <span>link: </span>
                    <span v-for="ip in ips" style="color: blue;cursor: pointer; margin-left: 10px;" @click="copy(ip)">
                        {{ 'ftp://' + ip + ':' + port }}
                    </span>
                </template>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from "vue";
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
    } catch (e) {
        ElMessage({ type: "error", message: e ? e.toString() : "未知错误" });
    }
}

</script>



<style lang="scss" scoped></style>