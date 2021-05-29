<template>
  <div class="view game">
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

<style lang="scss">
.game {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
