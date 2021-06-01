import { createApp } from 'vue';
import notifications from '@kyvg/vue3-notification';
import App from './App.vue';
import router from './router';
import store from './store';

createApp(App).use(store).use(router).use(notifications)
  .mount('#app');
