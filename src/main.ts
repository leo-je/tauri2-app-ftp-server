import { createApp } from "vue";
import App from "./App.vue";
import router from './router';
import i18n from './i18n';

import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import './styles/global.scss'


const app = createApp(App);
app.use(router);
app.use(i18n);
app.mount("#app");
