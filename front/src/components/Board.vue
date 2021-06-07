<template>
  <div
    class="board"
    :class="{
      rotated,
      'white-turn': whiteTurn,
      'no-transition': !doAnimations,
      'reverse-animation': reverseAnimation
    }"
  >
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
          @click="clickSquare(8 - row, col - 1)"
          :class="{ selected: isSquareSelected(8 - row, col - 1) }"
        ></div>
      </div>
    </div>
    <div class="pieces">
      <div
        class="piece"
        v-for="piece in pieces"
        :key="piece.id"
        :class="`
          row-${piece.row}
          col-${piece.column}
          ${piece.moving ? 'moving' : ''}
          ${piece.dead ? 'dead' : ''}
        `"
      ><img :src="getSquareImage(piece.type + piece.color)" :alt="piece.type + piece.color"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import Cell from '@/common/Cell';
import { Action } from '@/common/Action';
import Piece from '@/common/Piece';

export default defineComponent({
  name: 'Board',
  data() {
    return {
      selectedSquare: undefined as Cell | undefined,
      doAnimations: false,
      reverseAnimation: false,
    };
  },
  props: {
    pieces: {
      type: Array as PropType<Array<Piece>>,
      required: true,
    },
    rotated: {
      type: Boolean,
      default: false,
    },
    whiteTurn: {
      type: Boolean,
      required: true,
    },
    actions: {
      type: Array as PropType<Array<Action>>,
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
        const action: Action = new Action(this.selectedSquare, cell);
        this.$emit('movePiece', action);

        // unselect piece
        this.selectedSquare = undefined;
      }
    },
    isSquareSelected(row: number, column: number) {
      return row === this.selectedSquare?.row && column === this.selectedSquare?.column;
    },
  },
  watch: {
    pieces(pieces, oldPieces) {
      if (oldPieces.length > 0) {
        this.doAnimations = true;
      }
    },
    actions(actions, oldActions) {
      if (oldActions.length !== actions.length) {
        this.reverseAnimation = oldActions.length > actions.length;
      }
    },
  },
});
</script>

<style scoped lang="scss">
@use '../scss/theme';
@use 'sass:string';

$squareSize: 12.5%;
$max-board-size: 512px;
$border-width: 32px;
$border-width-small: 16px;

.no-transition {
  &, * {
    transition: none !important;
    animation-duration: 0s !important;
    animation-delay: 0s !important;
  }
}

.board {
  position: relative;
  max-height: $max-board-size;
  max-width: $max-board-size;
  width: 100%;
  border: $border-width solid theme.$board-border-color;
  box-sizing: border-box;
  box-shadow: 0 -10px 0 theme.$board-turn-indicator;
  transition: transform 2s, box-shadow 1s .5s;

  &.reverse-animation {
    .piece {
      transition: top 1s, left 1s;
      animation: unset !important;
    }
  }

  .background .row {
    &:first-child .square::before,
    &:last-child .square::before,
    &::before, &::after {
      transition: transform 2s;
    }
  }

  &.rotated {
    transform: rotate(180deg);

    .background .row {
      &:first-child .square::before,
      &:last-child .square::before,
      &::before, &::after {
        transform: rotate(-180deg);
      }
    }

    .piece img {
      transform: rotate(-180deg);
    }
  }

  &.white-turn {
    box-shadow: 0 10px 0 theme.$board-turn-indicator;
  }

  .background {
    display: grid;
    grid-template-rows: repeat(8, 1fr);

    .row {
      display: grid;
      grid-template-columns: repeat(8, 1fr);
      position: relative;

      &:nth-child(2n) .square:nth-child(2n),
      &:nth-child(2n+1) .square:nth-child(2n+1) {
        background: theme.$board-square-light;
      }

      .square {
        padding-bottom: 100%;
        position: relative;
        background: theme.$board-square-dark;

        &::after {
          content: '';
          position: absolute;
          left: 0;
          top: 0;
          right: 0;
          bottom: 0;
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

        &.selected:hover::after {
          background: #11ee1144;
        }
      }

      &:first-child, &:last-child {
        $letters: 'abcdefgh';
        @for $i from 1 through 8 {
          .square:nth-child(#{$i})::before {
            content: string.slice($letters, $i, $i);
          }
        }

        .square::before {
          position: absolute;
          z-index: 1;
          line-height: $border-width;
          opacity: 0.7;
          font-weight: bold;
          text-align: center;
          width: 100%;
        }
      }

      &:first-child .square::before {
        top: -$border-width;
      }

      &:last-child .square::before {
        bottom: -$border-width;
      }

      @for $i from 1 through 8 {
        &:nth-child(#{$i})::before, &:nth-child(#{$i})::after {
          content: string.quote(#{9-$i});
        }
      }

      &::before, &::after {
        position: absolute;
        z-index: 1;
        line-height: 0;
        top: 50%;
        opacity: 0.7;
        font-weight: bold;
        width: $border-width;
        text-align: center;
      }

      &::before {
        left: -$border-width;
      }

      &::after {
        right: -$border-width;
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
          top: (7-$i) * $squareSize;
        }

        &.col-#{$i} {
          left: $i * $squareSize;
        }
      }

      @keyframes raisePiece {
        0%   { transform: scale(1); }
        50%  { transform: scale(1.2); }
        100% { transform: scale(1); }
      }

      @keyframes vanishPiece {
        0%   {
          opacity: 1;
          transform: scale(1);
        }
        100% {
          opacity: 0;
          transform: scale(2);
        }
      }

      &.moving {
        transition: 1s;
        z-index: 1;
        animation: raisePiece 1s;
      }

      &.dead {
        opacity: 0;
        animation: vanishPiece 0.5s 0.5s both;
      }

      img {
        width: 100%;
        transition: transform 2s;
      }
    }
  }
}

@media screen and (max-width: $max-board-size) {
  .board {
    border-width: $border-width-small;
    transition: box-shadow 1s .5s;

    .background .row {
      &:first-child .square::before {
        top: -$border-width-small;
        line-height: $border-width-small;
        font-size: 0.7em;
        transition: none;
      }

      &:last-child .square::before {
        bottom: -$border-width-small;
        line-height: $border-width-small;
        font-size: 0.7em;
        transition: none;
      }

      &::before, &::after {
        width: $border-width-small;
        font-size: 0.7em;
        transition: none;
      }

      &::before {
        left: -$border-width-small;
      }

      &::after {
        right: -$border-width-small;
      }
    }

    .pieces .piece img {
      transition: none;
    }
  }
}
</style>
