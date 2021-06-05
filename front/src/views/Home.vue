<template>
  <div class="view home">
    <h1>Web-chess</h1>
    <p>Welcome.</p>
    <button @click="newGame">New game</button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import api from '@/api';

export default defineComponent({
  name: 'Home',
  methods: {
    async newGame() {
      const id : number = await api.createGame();
      this.$router.push(`/game/${id}`);
    },
  },
});
</script>
