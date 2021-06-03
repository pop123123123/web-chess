<template>
  <div class="view game">
    <p><label><input type="checkbox" v-model="rotatedBoard">Rotate board</label></p>
    <Board
      :pieces="state.pieces"
      :rotated="rotatedBoard"
      :white-turn="whiteTurn"
      @movePiece="sendAction"
    />
    <GameNotifications />
  </div>
  <footer>
    <p>Share this game: <Copy :text="url"/></p>
  </footer>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Board from '@/components/Board.vue';
import Copy from '@/components/Copy.vue';
import GameNotifications from '@/components/GameNotifications.vue';
import Action from '@/common/Action';
import Game from '@/common/Game';
import Piece from '@/common/Piece';
import api from '@/api';

export default defineComponent({
  name: 'Game',
  props: {
    id: Number,
  },
  components: {
    Board,
    Copy,
    GameNotifications,
  },
  data() {
    return {
      polling: 0,
      state: {
        pieces: [] as Piece[],
      },
      game: undefined as Game | undefined,
      rotatedBoard: false,
    };
  },
  computed: {
    url() {
      return window.location.href;
    },
    whiteTurn(): boolean {
      return (this.game?.history.length ?? 0) % 2 === 0;
    },
  },
  methods: {
    async updateBoard() {
      const gameId = this.game?.id ?? parseInt(this.$route.params.id as string, 10);
      try {
        this.game = await api.getGame(gameId);
        this.state.pieces = this.game.getPieces();
      } catch (error) {
        if (error.response?.status === 404) {
          this.stopPolling();
          this.$notify({
            title: 'This game does not exist',
            type: 'error',
            duration: -1,
          });
        } else {
          this.$notify({
            title: 'Unknown error',
            type: 'error',
          });
        }
      }
    },
    async sendAction(action: Action) {
      if (this.game === undefined) { return; }
      try {
        await api.sendAction(this.game.id, action);
        await this.updateBoard();
      } catch (error) {
        if (error.response?.status === 400) {
          this.$notify({
            title: 'Illegal move!',
            type: 'warn',
          });
        } else if (error.response?.status === 403) {
          this.$notify({
            title: 'Not your turn!',
            type: 'warn',
          });
        } else {
          this.$notify({
            title: 'Unknown error',
            type: 'error',
          });
        }
      }
    },
    startPolling() {
      this.polling = setInterval(() => {
        if (this.polling) {
          this.updateBoard();
        }
      }, 1000);
    },
    stopPolling() {
      clearInterval(this.polling);
      this.polling = 0;
    },
  },
  async mounted() {
    this.startPolling();
    await this.updateBoard();
  },
  unmounted() {
    this.stopPolling();
  },
});
</script>

<style lang="scss">
.game {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  user-select: none;
  overflow: hidden;
  position: relative;
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
