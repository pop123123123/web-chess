<template>
  <div class="view game">
    <Board
      :state="state.board"
      @movePiece="sendAction"
    />
  </div>
  <footer>
    <p>Share this game: <Copy :text="url"/></p>
  </footer>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Board from '@/components/Board.vue';
import Copy from '@/components/Copy.vue';
import Action from '@/common/Action';
import Game, { EMPTY_BOARD } from '@/common/Game';
import api from '@/api';

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
      polling: undefined as number | undefined,
      state: {
        board: EMPTY_BOARD,
      },
      game: undefined as Game | undefined,
    };
  },
  computed: {
    url() {
      return window.location.href;
    },
  },
  methods: {
    async updateBoard() {
      const gameId = this.game?.id ?? parseInt(this.$route.params.id as string, 10);
      this.game = await api.getGame(gameId);
      this.state.board = this.game.getBoard();
    },
    async sendAction(action: Action) {
      if (this.game === undefined) { return; }
      await api.sendAction(this.game.id, action);
      await this.updateBoard();
    },
  },
  async mounted() {
    await this.updateBoard();
    this.polling = setInterval(() => {
      this.updateBoard();
    }, 1000);
  },
  unmounted() {
    clearInterval(this.polling);
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
