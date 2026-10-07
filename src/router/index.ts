import { createRouter, createWebHistory } from 'vue-router';
import Ftp from '../view/Ftp.vue';
import FtpAuth from '../view/FtpAuth.vue';
import About from '../view/About.vue';

/**
 * 三个页面都在同一个窗口里，切换由侧边栏驱动。
 * 用 KeepAlive 缓存，服务状态和日志滚动位置不会因为切页丢失。
 */
const routes = [
  {
    path: '/',
    name: 'server',
    component: Ftp
  },
  {
    path: '/auth',
    name: 'auth',
    component: FtpAuth
  },
  {
    path: '/about',
    name: 'about',
    component: About
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
