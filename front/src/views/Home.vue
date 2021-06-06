<template>
  <div class="view home">
    <h1>Web-chess</h1>
    <p>Welcome.</p>
    <button @click="newGame">New game</button>
    <form @submit="importGame">
      <input id="jsondata" type="text" v-model="jsonData"/>
      <label for="jsondata">Game data</label>
      <input type="submit" value="Import game"/>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import api from '@/api';
import { historyFromBase64 } from '@/common/b64';

export default defineComponent({
  name: 'Home',
  data: () => ({
    jsonData: '',
  }),
  methods: {
    async newGame() {
      const id : number = await api.createGame();
      this.$router.push(`/game/${id}`);
    },
    async importGame(e: Event) {
      e.preventDefault();
      try {
        const history = historyFromBase64(this.jsonData);
        const id : number = await api.importGame(history);
        this.$router.push(`/game/${id}`);
      } catch (_e) {
        this.$notify({
          title: 'This game is invalid',
          type: 'error',
        });
      }
    },
  },
});
</script>
