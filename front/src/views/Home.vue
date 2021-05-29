<template>
  <div class="home">
    <h1>Web-chess</h1>
    <p>{{message}}</p>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import axios from 'axios';

export default defineComponent({
  name: 'Home',
  data: () => ({
    message: '',
  }),
  async mounted() {
    const message = (await axios.get('/message')).data;
    this.message = message;
  },
});
</script>
