import { createApp } from 'vue';
import notifications from '@kyvg/vue3-notification';
import FontAwesomeIcon from '@/font-awesome';
import App from './App.vue';
import router from './router';
import store from './store';

createApp(App).use(store).use(router).use(notifications)
  .component('font-awesome-icon', FontAwesomeIcon)
  .mount('#app');
