<template>
  <div class="view game">
    <Board :state="state.board" />
  </div>
  <footer>
    <p>Share this game: <Copy :text="url"/></p>
  </footer>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Board from '@/components/Board.vue';
import Copy from '@/components/Copy.vue';

export default defineComponent({
  name: 'Game',
  props: {
    id: Number,
  },
  components: {
    Board,
    Copy,
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
  computed: {
    url() {
      return window.location.href;
    },
  },
});
</script>

<style lang="scss">
.game {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  user-select: none;
}
footer {
  $h: 3rem;
  width: 100%;
  height: $h;
  line-height: $h;
  background: #444;
  padding: 1em 0;
  text-align: center;
  p {
    margin: 0;
  }
}
</style>
