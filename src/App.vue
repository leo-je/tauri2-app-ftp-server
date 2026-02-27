<template>
  <el-container class="app-container">
    <el-main class="index-main" @contextmenu="disableContextMenu">
      <RouterView />
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ElMain, ElContainer } from 'element-plus';
function disableContextMenu(event: any) {
   event.preventDefault();
}
</script>

<style lang="scss" scoped>
.app-container {
  height: 100%;
  width: 100%;
  background: var(--gradient-bg);
  background-size: 400% 400%;
  animation: gradientShift 15s ease infinite;
  position: relative;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
    animation: float 20s ease-in-out infinite;
  }
}

@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

@keyframes float {
  0%, 100% {
    transform: translate(0, 0) rotate(0deg);
  }
  33% {
    transform: translate(30px, -30px) rotate(120deg);
  }
  66% {
    transform: translate(-20px, 20px) rotate(240deg);
  }
}

.index-main {
  color: var(--el-text-color-primary);
  height: 100%;
  width: 100%;
  padding: 0;
  overflow: auto;
  position: relative;
  z-index: 1;
}

* {
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>
