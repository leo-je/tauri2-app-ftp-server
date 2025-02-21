// src/router.js
import { createRouter, createWebHistory } from 'vue-router';
import ftp from '../view/Ftp.vue';
import ftpTabs from '../view/FtpTabs.vue';

const routes = [
  {
    path: '/',
    name: 'ftpTabs',
    component: ftpTabs
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;