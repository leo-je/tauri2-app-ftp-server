<template>
    <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
        <el-tab-pane label="基本" name="base">
            <ftp></ftp>
        </el-tab-pane>
        <el-tab-pane label="权限" name="auth">
            <ftpAuth></ftpAuth>
        </el-tab-pane>
        <!-- <el-tab-pane label="关于" name="about">关于应用</el-tab-pane> -->
        <!-- <el-tab-pane label="Task" name="fourth">Task</el-tab-pane> -->
    </el-tabs>
</template>
<script lang="ts" setup>
import { ref } from 'vue'
import type { TabsPaneContext } from 'element-plus'
import { ElTabs, ElTabPane } from "element-plus";
import ftp from './Ftp.vue'
import ftpAuth from './FtpAuth.vue'

const activeName = ref('base')

const handleClick = (tab: TabsPaneContext, event: Event) => {
    console.log(tab, event)
}
</script>

<style scoped>
.demo-tabs>.el-tabs__content {
    padding: 32px;
    color: #6b778c;
    font-size: 32px;
    font-weight: 600;
}
</style>