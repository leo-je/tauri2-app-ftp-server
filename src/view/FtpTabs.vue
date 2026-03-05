<template>
    <div class="tabs-container">
        <el-tabs v-model="activeName" class="ftp-tabs" @tab-click="handleClick">
            <el-tab-pane name="base">
                <template #label>
                    <span class="tab-label">
                        <SvgIcon name="folder" :size="18" class="tab-icon" />
                        {{ $t('tabs.server') }}
                    </span>
                </template>
                <ftp></ftp>
            </el-tab-pane>
            <el-tab-pane name="auth">
                <template #label>
                    <span class="tab-label">
                        <SvgIcon name="lock" :size="18" class="tab-icon" />
                        {{ $t('tabs.auth') }}
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
import { SvgIcon } from '../components/icons';
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
    padding: 0 10px;
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