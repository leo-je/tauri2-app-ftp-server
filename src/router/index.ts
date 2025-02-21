// src/router.js
import { createRouter, createWebHistory } from 'vue-router';
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