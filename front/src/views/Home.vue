<template>
  <div class="view home">
    <h1>Web-chess</h1>
    <p>{{message}}</p>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import api from '@/api';

export default defineComponent({
  name: 'Home',
  data: () => ({
    message: '',
  }),
  async mounted() {
    const message = await api.getMessage();
    this.message = message;
  },
});
</script>
