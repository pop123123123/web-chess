<template>
  <div class="view home">
    <div class="card">
      <p>Welcome to</p>
      <h1>Web-chess</h1>
      <p>A chess game made with Rust and Vue.js</p>
      <TitleSeparator>Create a new game</TitleSeparator>
      <Button class="success" icon="chess" @click="newGame">Play</Button>
      <TitleSeparator>Import a past game</TitleSeparator>
      <form @submit="importGame">
        <label for="jsondata">Enter the exported game string:</label>
        <input id="jsondata" type="text" v-model="jsonData">
        <Button icon="chess" :disabled="jsonData.length <= 0">Import</Button>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { historyFromBase64 } from '@/common/b64';
import Button from '@/components/Button.vue';
import TitleSeparator from '@/components/TitleSeparator.vue';

export default defineComponent({
  name: 'Home',
  components: {
    Button,
    TitleSeparator,
  },
  data: () => ({
    jsonData: '',
  }),
  methods: {
    async newGame() {
      const id : number = await this.$store.dispatch('CREATE_GAME');
      this.$router.push(`/game/${id}`);
    },
    async importGame(e: Event) {
      e.preventDefault();
      try {
        const history = historyFromBase64(this.jsonData);
        const id : number = await this.$store.dispatch('IMPORT_GAME', history);
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

<style lang="scss" scoped>
@use '../scss/theme';

.home {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.card {
  display: flex;
  flex-direction: column;
  justify-content: stretch;
  padding: 20px 40px 35px;
  background: theme.$background-secondary;
  border-radius: 8px;
  min-width: 16em;
  text-align: center;
  gap: 5px;

  h1 {
    margin: 0;
  }

  p {
    opacity: 0.7;
  }

  form {
    display: flex;
    flex-direction: column;
    justify-content: stretch;

    label {
      font-size: 0.7em;
      font-weight: bold;
      text-align: left;
      padding: 0 10px;
      opacity: 0.8;
    }

    input[type=text] {
      background: theme.$background-header;
      border-radius: 8px;
      border: 3px solid #00000066;
      font: inherit;
      color: theme.$color-text-dark;
      padding: 5px;
      margin: 5px 0 10px;
      opacity: 0.8;

      &:focus {
        opacity: 1;
      }
    }
  }
}
</style>
