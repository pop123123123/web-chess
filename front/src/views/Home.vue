<template>
  <div class="view home">
    <h1>Web-chess</h1>
    <p>{{message}}</p>
    <button @click="newGame">New game</button>
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
    const message : string = await api.getMessage();
    this.message = message;
  },
  methods: {
    async newGame() {
      const id : number = await api.createGame();
      this.$router.push(`/game/${id}`);
    },
  },
});
</script>
