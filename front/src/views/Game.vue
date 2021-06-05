<template>
  <div class="wrapper">
    <div class="view game">
      <aside>
        <History :history="history"/>
      </aside>
      <div class="board-container">
        <Board
          :pieces="state.pieces"
          :rotated="rotatedBoard"
          :white-turn="whiteTurn"
          :actions="game?.history ?? []"
          @movePiece="sendAction"
        />
        <GameNotifications />
      </div>
      <aside>
        <ul class="buttons">
          <li><Switch
            theme="black-white"
            off-text="White"
            on-text="Black"
            v-model:value="rotatedBoard"
          /></li>
          <li><Button class="success" @click="resetGame">Reset game</Button></li>
          <li><Button class="success">Offer draw</Button></li>
          <li><Button class="success" @click="deleteLastAction">Undo</Button></li>
        </ul>
      </aside>
    </div>
    <footer>
      <p>Share this game: <Copy :text="url"/></p>
    </footer>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Board from '@/components/Board.vue';
import Button from '@/components/Button.vue';
import Copy from '@/components/Copy.vue';
import History from '@/components/History.vue';
import GameNotifications from '@/components/GameNotifications.vue';
import Switch from '@/components/Switch.vue';
import { Action } from '@/common/Action';
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
    Button,
    Copy,
    History,
    GameNotifications,
    Switch,
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
    history(): string[] {
      return (this.game?.history ?? []).map((a) => a.toAlgebraicNotation());
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
    async resetGame() {
      if (this.game === undefined) { return; }
      try {
        await api.resetGame(this.game.id);
        await this.updateBoard();
      } catch (error) {
        this.$notify({
          title: 'Unknown error',
          type: 'error',
        });
      }
    },
    async deleteLastAction() {
      if (this.game === undefined) { return; }
      try {
        await api.deleteLastAction(this.game.id);
        await this.updateBoard();
      } catch (error) {
        if (error.response?.status === 400) {
          this.$notify({
            title: 'Cannot undo!',
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
@use 'sass:math';
@use '../scss/theme';

$marginSides: 2vw;
$gameHeight: 512px;

.wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.game {
  flex: 1;
  display: flex;
  padding: 0;

  .board-container {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    user-select: none;
    overflow: hidden;
    position: relative;
    padding: 10px;
  }

  aside {
    display: flex;
    flex-direction: column;
    justify-content: center;
    background: theme.$background-secondary;

    .buttons {
      display: flex;
      flex-direction: column;
      align-items: stretch;
      justify-content: space-around;
      margin: 0;
      list-style: none;
      gap: 20px;
      padding: 10px 40px;

      li {
        button {
          width: 100%;
        }
      }
    }
  }
}
footer {
  $h: 3rem;
  background: theme.$background-footer;
  padding: 0.5em 0;
  text-align: center;
  overflow: hidden;

  p {
    margin: 0;
  }
}

@media screen and (max-width: 900px) {
  .game {
    flex-direction: column;

    aside > .buttons {
      flex-direction: row;
      justify-content: space-between;
      align-items: center;
      gap: 5px;
      flex-wrap: wrap;
      padding: 10px;

      li {
        min-width: initial;
        flex: 1;
      }
    }
  }
}
</style>
