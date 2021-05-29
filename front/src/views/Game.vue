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
          ['rd', 'nd', 'bd', 'qd', 'kd', 'bd', 'nd', 'rd'],
          ['pd', 'pd', 'pd', 'pd', 'pd', 'pd', 'pd', 'pd'],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
          ['pl', 'pl', 'pl', 'pl', 'pl', 'pl', 'pl', 'pl'],
          ['rl', 'nl', 'bl', 'ql', 'kl', 'bl', 'nl', 'rl'],
        ],
      },
    };
  },
});
</script>
