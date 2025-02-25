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

            <!-- <div style="margin-top: 10px;margin-bottom: 10px;">
                <span>log:</span><span>xxxxxx</span>
            </div> -->
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { ElButton, ElMessage, ElInput } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { Command } from 'tauri-plugin-shellx-api';
import { platform } from '@tauri-apps/plugin-os';
import { Store } from '@tauri-apps/plugin-store';
import { info, error, attachConsole } from '@tauri-apps/plugin-log';

//import { appDataDir } from '@tauri-apps/api/path';

let store: Store;//await Store.load('store.json');

const dirPath = ref('')
const port = ref(21)
const isStart = ref(false)

async function init() {
    const detach = await attachConsole();
    detach()
    // 从localStorage中检索数据
    store = await Store.load('store.json');
    var selected = await store.get('selected');// localStorage.getItem('selected');
    if (selected) {
        info(selected + '')
        dirPath.value = selected + ''
    }
}


onMounted(init)
const logl = (msg: string) => {
    info(msg + '/n')
}

const checkPlatform = () => {
    let currentPlatform = platform();
    if (currentPlatform === 'windows') {
        logl('当前操作系统是 Windows');
    } else if (currentPlatform === 'macos') {
        logl('当前操作系统是 macOS');
    } else if (currentPlatform === 'linux') {
        logl('当前操作系统是 Linux');
    } else {
        logl('未知操作系统：' + currentPlatform);
    }
    return currentPlatform;
}

async function openDir() {
    let path = dirPath.value
    if (!path || path == '') {
        ElMessage({ type: "warning", message: "未选择目录" })
        return;
    }
    let osType = checkPlatform()
    new Command(osType === 'windows' ? 'explorer' : 'open', [path]).execute()
}

async function selectDir() {
    try {
        //console.log(appDataDir())
        const selected = await open({
            directory: true,
            multiple: false,
            // defaultPath: await appDataDir(),
        }) as string;
        console.log(selected)
        if (selected && selected != '') {
            dirPath.value = selected
            // 存储数据到localStorage
            // localStorage.setItem('selected', selected);
            store.set('selected', selected);
        }
    } catch (e) {
        error(e + '');
        ElMessage(e + '')
    }

}


function startOrStopServer() {
    let b = isStart.value
    console.log(b)
    if (b) {
        stopFtpServer()
    } else {
        startFtpServer()
    }
}

async function stopFtpServer() {
    try {
        const result = await invoke('stop_ftp_server', {});
        ElMessage({ type: "success", message: result + "" }); // 处理返回结果
        isStart.value = false
    } catch (error) {
        ElMessage({ type: "error", message: error + "" }); // 处理错误
    }
}

async function startFtpServer() {
    try {
        let path = dirPath.value
        if (path == null || path == '') {
            ElMessage("请选择路径")
            return;
        }
        info("invoke-'start_ftp_server'")
        let users = await store.get('tableData')
        if(!users){
            users = []
        }
        console.log('user:' + JSON.stringify(users))
        let isAnonymous = await store.get('isAnonymous')
        console.log(isAnonymous)

        let fileauth = await store.get('fileauth')
        console.log('fileauth:' + fileauth)
        if (!fileauth) fileauth = 'R'

        const result = await invoke('start_ftp_server', {
            path: dirPath.value, port: port.value + "",
            users: JSON.stringify(users), isAnonymous: isAnonymous ? isAnonymous : false,
            fileauth
        });
        ElMessage({ type: "success", message: result + "" }); // 处理返回结果
        isStart.value = true
    } catch (error) {
        ElMessage({ type: "error", message: error + "" }); // 处理错误
    }
}

</script>



<style lang="scss" scoped></style>