<template>
  <div class="board">
    <div class="background">
      <div
        class="row"
        v-for="row in 8"
        :key="row"
      >
        <div
          class="square"
          v-for="col in 8"
          :key="col"
          @click="clickSquare(row - 1, col - 1)"
          :class="{ selected: isSquareSelected(row - 1, col - 1) }"
        ></div>
      </div>
    </div>
    <div class="pieces">
      <div
        class="piece"
        v-for="(piece, pieceIndex) in pieces"
        :key="pieceIndex"
        :class="`row-${piece.row} col-${piece.column}`"
      ><img :src="getSquareImage(piece.type + piece.color)" :alt="piece.type + piece.color"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import Cell from '@/common/Cell';
import Action from '@/common/Action';
import Piece from '@/common/Piece';

export default defineComponent({
  name: 'Board',
  data() {
    return {
      selectedSquare: undefined as Cell | undefined,
    };
  },
  props: {
    pieces: {
      type: Array as PropType<Array<Piece>>,
      required: true,
    },
  },
  methods: {
    getSquareImage(square: string) {
      const images = require.context('../assets/pieces/', false, /\.svg$/);
      if (square.match(/[bknpqr][dl]/)) {
        return images(`./${square}.svg`);
      }
      return null;
    },
    clickSquare(row: number, column: number) {
      const cell = { row, column } as Cell;
      if (this.selectedSquare === undefined) {
        // select piece
        if (this.pieces.find(
          (piece) => piece.row === cell.row && piece.column === cell.column,
        ) !== undefined) {
          this.selectedSquare = cell;
        }
      } else if (cell.row === this.selectedSquare.row
      && cell.column === this.selectedSquare.column) {
        // unselect piece
        this.selectedSquare = undefined;
      } else {
        // move selected piece to position
        const action: Action = { from: this.selectedSquare, to: cell };
        this.$emit('movePiece', action);

        // unselect piece
        this.selectedSquare = undefined;
      }
    },
    isSquareSelected(row: number, column: number) {
      return row === this.selectedSquare?.row && column === this.selectedSquare?.column;
    },
  },
});
</script>

<style scoped lang="scss">
$squareSize: 64px;

.board {
  position: relative;
  width: $squareSize * 8;
  height: $squareSize * 8;
  overflow: hidden;
  border: 10px solid black;

  .background {
    display: grid;
    grid-template-rows: repeat(8, 1fr);

    .row {
      display: grid;
      height: $squareSize;
      grid-template-columns: repeat(8, 1fr);

      &:nth-child(2n) .square:nth-child(2n),
      &:nth-child(2n+1) .square:nth-child(2n+1) {
        background: wheat;
      }

      .square {
        position: relative;
        background: peru;

        &::after {
          content: '';
          position: absolute;
          left: 0;
          top: 0;
          right: 0;
          bottom: 0;
          height: $squareSize;
          z-index: 1;
          display: none;
        }

        &:hover::after {
          display: block;
          background: #1199dd33;
        }

        &.selected::after {
          display: block;
          background: #11ee1166;
        }
      }
    }
  }

  .pieces {
    pointer-events : none;

    .piece {
      position: absolute;
      width: $squareSize;
      height: $squareSize;

      @for $i from 0 through 7 {
        &.row-#{$i} {
          top: $i * $squareSize;
        }

        &.col-#{$i} {
          left: $i * $squareSize;
        }
      }

      img {
        width: 100%;
      }
    }
  }
}
</style>
