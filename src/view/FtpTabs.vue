<template>
    <div class="tabs-container">
        <el-tabs v-model="activeName" class="ftp-tabs" @tab-click="handleClick">
            <el-tab-pane name="base">
                <template #label>
                    <span class="tab-label">
                        <svg class="tab-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M3 7V17C3 18.1046 3.89543 19 5 19H19C20.1046 19 21 18.1046 21 17V9C21 7.89543 20.1046 7 19 7H13L11 5H5C3.89543 5 3 5.89543 3 7Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                        服务器
                    </span>
                </template>
                <ftp></ftp>
            </el-tab-pane>
            <el-tab-pane name="auth">
                <template #label>
                    <span class="tab-label">
                        <svg class="tab-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M12 15V17M6 21H18C19.1046 21 20 20.1046 20 19V13C20 11.8954 19.1046 11 18 11H6C4.89543 11 4 11.8954 4 13V19C4 20.1046 4.89543 21 6 21ZM16 11V7C16 4.79086 14.2091 3 12 3C9.79086 3 8 4.79086 8 7V11H16Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                        权限管理
                    </span>
                </template>
                <ftpAuth></ftpAuth>
            </el-tab-pane>
        </el-tabs>
    </div>
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

<style lang="scss" scoped>
.tabs-container {
    height: 100%;
    padding: 20px;
    animation: fadeIn 0.5s ease;
}

.ftp-tabs {
    height: 100%;
    
    :deep(.el-tabs__header) {
        margin-bottom: 24px;
        border-bottom: 2px solid rgba(102, 126, 234, 0.1);
    }
    
    :deep(.el-tabs__nav-wrap::after) {
        display: none;
    }
    
    :deep(.el-tabs__item) {
        font-size: 15px;
        font-weight: 500;
        color: rgba(255, 255, 255, 0.7);
        padding: 0 28px;
        height: 48px;
        line-height: 48px;
        transition: all 0.3s ease;
        
        &:hover {
            color: rgba(255, 255, 255, 0.9);
        }
        
        &.is-active {
            color: white;
            font-weight: 600;
            text-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
        }
    }
    
    :deep(.el-tabs__active-bar) {
        background: linear-gradient(90deg, rgba(255, 255, 255, 0.8), rgba(255, 255, 255, 0.4));
        height: 3px;
        border-radius: 2px;
    }
    
    :deep(.el-tabs__content) {
        height: calc(100% - 72px);
        overflow: auto;
    }
    
    :deep(.el-tab-pane) {
        height: 100%;
    }
}

.tab-label {
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.tab-icon {
    width: 18px;
    height: 18px;
    stroke-width: 2;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
</style>