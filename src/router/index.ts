// src/router.js
import { createRouter, createWebHistory } from 'vue-router';
import ftp from '../view/Ftp.vue';

const routes = [
  {
    path: '/',
    name: 'ftp',
    component: ftp
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;