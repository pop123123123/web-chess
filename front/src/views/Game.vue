<template>
  <div v-if="connectError" class="view game-connect-error">
    <img class="icon" src="@/assets/error.svg" alt="Not found">
    Error when connecting. Retrying...
  </div>
  <div v-else-if="loading" class="view game-loading"></div>
  <div v-else-if="notFound" class="view game-not-found">
    <img class="icon" src="@/assets/not-found.svg" alt="Not found">
    This game does not exist.
  </div>
  <div v-else class="wrapper">
    <div class="view game">
      <aside>
        <History :actions="actions" v-model:hovered-action="previewedAction"/>
      </aside>
      <div class="board-container">
        <Board
          :pieces="state.pieces"
          :rotated="rotatedBoard"
          :white-turn="whiteTurn"
          :previewed-action="previewedAction"
          @movePiece="movePiece"
          @confirmPromotion="confirmPromotion"
        />
      </div>
      <aside><div>
        <ul class="buttons">
          <li><Switch
            theme="black-white"
            off-text="White"
            on-text="Black"
            v-model:value="rotatedBoard"
          /></li>
          <li><Button class="success" icon="trash" @click="resetGame">Reset game</Button></li>
          <!-- <li><Button class="success" icon="handshake">Offer draw</Button></li> -->
          <li><Button class="success" icon="undo-alt" @click="deleteLastAction">Undo</Button></li>
        </ul>
        <div class="buttons-bottom">
          <div>
            <TitleSeparator>Invite players</TitleSeparator>
            <Share button-title="Share" title="Join me for a chess game!" :url="url"/>
          </div>
          <div>
            <TitleSeparator>Save game</TitleSeparator>
            <Share
              button-title="Export"
              icon="copy"
              :title="`Import this game on ${host}`"
              :text="jsonString"
            />
          </div>
        </div>
      </div></aside>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Board from '@/components/Board.vue';
import Button from '@/components/Button.vue';
import Share from '@/components/Share.vue';
import History from '@/components/History.vue';
import Switch from '@/components/Switch.vue';
import { Action } from '@/common/Action';
import Piece, { PieceColor, PieceType } from '@/common/Piece';
import TitleSeparator from '@/components/TitleSeparator.vue';
import Game from '@/common/Game';

export default defineComponent({
  name: 'Game',
  props: {
    id: Number,
  },
  components: {
    Board,
    Button,
    Share,
    History,
    Switch,
    TitleSeparator,
  },
  data() {
    return {
      polling: 0,
      loading: true,
      notFound: false,
      connectError: false,
      state: {
        pieces: [] as Piece[],
      },
      rotatedBoard: false,
      previewedAction: undefined as Action | undefined,
    };
  },
  computed: {
    url() {
      return window.location.href;
    },
    jsonString(): string {
      return this.game?.toBase64() ?? '';
    },
    whiteTurn(): boolean {
      return (this.game?.history.length ?? 0) % 2 === 0;
    },
    history(): string[] {
      return (this.game?.history ?? []).map((a) => a.toAlgebraicNotation());
    },
    host(): string {
      return window.location.host;
    },
    game(): Game | undefined {
      return this.$store.state.game;
    },
    actions(): Action[] {
      return this.game?.history ?? [];
    },
  },
  methods: {
    async updateBoard() {
      const gameId = this.game?.id ?? parseInt(this.$route.params.id as string, 10);
      try {
        await this.$store.dispatch('GET_GAME', gameId);
        if (this.game !== undefined) {
          this.state.pieces = this.game.getPieces();
        }
      } catch (error) {
        if (error.response?.status === 404) {
          this.stopPolling();
          this.notFound = true;
        } else {
          this.connectError = true;
          return;
        }
      }
      this.loading = false;
      this.connectError = false;
    },
    async movePiece(action: Action) {
      // get moving piece
      const piece = this.state.pieces.find(
        (p) => p.column === action.from.column && p.row === action.from.row,
      );

      if (piece?.type === PieceType.Pawn
      && ((this.whiteTurn && action.from.row === 6 && action.to.row === 7
      && piece.color === PieceColor.Light)
      || (!this.whiteTurn && action.from.row === 1 && action.to.row === 0
      && piece.color === PieceColor.Dark))
      && Math.abs(action.from.column - action.to.column) <= 1) {
        // promotion, check if move is valid
        if (await this.sendAction(action, true)) {
          // start promotion phase
          this.startPromotion(action, piece.color);
        }
      } else {
        // other cases
        await this.sendAction(action);
      }
    },
    async confirmPromotion(action: Action, pieceType: PieceType) {
      await this.sendAction(action, false, pieceType);
    },
    startPromotion(action: Action, color: PieceColor) {
      this.$store.commit('SET_PROMOTION', { action, color });
    },
    async sendAction(action: Action, dryRun = false, promotion?: PieceType): Promise<boolean> {
      if (this.game === undefined) { return false; }
      try {
        if (promotion) {
          const payload = { gameId: this.game.id, action, pieceType: promotion };
          await this.$store.dispatch('SEND_PROMOTION_ACTION', payload);
        } else {
          const payload = { gameId: this.game.id, action };
          await this.$store.dispatch(dryRun ? 'TRY_ACTION' : 'SEND_ACTION', payload);
        }
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
        } else if (error.response?.status === 404) {
          this.stopPolling();
          this.notFound = true;
        } else {
          this.$notify({
            title: 'Unknown error',
            type: 'error',
          });
        }
        return false;
      }
      return true;
    },
    async resetGame() {
      if (this.game === undefined) { return; }
      try {
        await this.$store.dispatch('RESET_GAME', this.game.id);
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
        await this.$store.dispatch('DELETE_LAST_ACTION', this.game.id);
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
  async unmounted() {
    this.stopPolling();
    await this.$store.dispatch('CLEAR_GAME_STATE');
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

.game-not-found, .game-connect-error {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  gap: 1em;

  > .icon {
    width: 8em;
    opacity: 0.7;
  }
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
    padding: 20px;
  }

  aside {
    background: theme.$background-secondary;
    overflow: hidden auto;
    max-height: calc(100vh - 56px);

    > div {
      min-height: 100%;
      display: flex;
      flex-direction: column;
    }

    .buttons {
      display: flex;
      flex-direction: column;
      align-items: stretch;
      justify-content: center;
      flex: 1;
      margin: 0;
      list-style: none;
      gap: 20px;
      padding: 20px 40px 10px;

      li {
        button {
          width: 100%;
        }
      }
    }

    .buttons-bottom {
      display: flex;
      flex-direction: column;
      align-items: stretch;
      padding: 0 40px 20px;
      overflow: hidden;

      button {
        width: 100%;
      }
    }
  }
}

@media screen and (max-width: 900px) {
  .game {
    flex-direction: column;

    .board-container {
      padding: 10px 0;
    }

    aside {
      max-height: unset;
      .buttons {
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        gap: 5px;
        flex-wrap: wrap;
        padding: 10px;
        padding-bottom: 0;

        li {
          flex: 1;
        }
      }

      .buttons-bottom {
        flex-direction: row;
        flex-wrap: wrap;
        row-gap: 0;
        padding: 0 7.5px 20px;

        > div {
          overflow: hidden;
          padding: 2.5px;
          flex: 1;
        }
      }
    }
  }
}
</style>
