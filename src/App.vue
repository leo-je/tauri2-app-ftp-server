<template>
  <el-container>
    <el-main class="index-main" style="overflow: auto;" @contextmenu="disableContextMenu">
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
* {
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  /* margin-top: 60px; */
}

html,
body,
#app,
.el-container {
  margin: 0;
  padding: 0;
  height: 100%;
}

.index-main {
  // background-color: #e9eef3;
  color: var(--el-text-color-primary);
  text-align: center;
  // line-height: 160px;
  height: 100%;
  width: 100%;
  padding: 0px;
}
</style>
