<template>
  <div class="game">
    <h1>Game</h1>
    <Board :state="state.board" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Board from '@/components/Board.vue';

export default defineComponent({
  name: 'Game',
  components: {
    Board,
  },
  data() {
    return {
      state: {
        board: [
          ['rd', 'kd', 'bd', 'qd', 'kd', 'bd', 'kd', 'rd'],
          ['pd', 'pd', 'pd', 'pd', 'pd', 'pd', 'pd', 'pd'],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['pl', 'pl', 'pl', 'pl', 'pl', 'pl', 'pl', 'pl'],
          ['rl', 'kl', 'bl', 'ql', 'kl', 'bl', 'kl', 'rl'],
        ],
      },
    };
  },
});
</script>
