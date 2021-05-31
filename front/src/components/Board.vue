<template>
  <div class="board">
    <div
      class="row"
      v-for="(row, rowIndex) in rows"
      :key="rowIndex"
    >
      <div
        class="square"
        v-for="(square, colIndex) in row"
        :key="colIndex"
        @click="clickSquare(rowIndex, colIndex)"
        :class="{ selected: isSquareSelected(rowIndex, colIndex) }"
      ><img :src="getSquareImage(square)" :alt="square"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import Cell from '@/common/Cell';

export default defineComponent({
  name: 'Board',
  data() {
    return {
      selectedSquare: undefined as Cell | undefined,
    };
  },
  props: {
    state: {
      type: Array as PropType<Array<Array<string>>>,
      required: true,
    },
  },
  computed: {
    rows(): Array<Array<string>> {
      return this.state;
    },
  },
  methods: {
    getSquareImage(square: string) {
      const images = require.context('../assets/pieces/', false, /\.svg$/);
      if (square.match(/[bknpqr][dl]/)) {
        return images(`./${square}.svg`);
      }
      return images('./empty.svg');
    },
    clickSquare(row: number, column: number) {
      const cell = { row, column } as Cell;
      if (this.selectedSquare === undefined) {
        // select piece
        this.selectedSquare = cell;
      } else if (cell.row === this.selectedSquare.row
      && cell.column === this.selectedSquare.column) {
        // unselect piece
        this.selectedSquare = undefined;
      } else {
        // move selected piece to position
        // TODO: send action

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
.board {
  display: grid;
  width: 512px;
  height: 512px;
  grid-template-rows: repeat(8, 1fr);
  background: black;
  overflow: hidden;
  border: 10px solid black;

  .row {
    display: grid;
    height: 64px;
    grid-template-columns: repeat(8, 1fr);

    &:nth-child(2n) .square:nth-child(2n),
    &:nth-child(2n+1) .square:nth-child(2n+1) {
      background: wheat;
    }

    .square {
      position: relative;
      background: peru;

      img {
        width: 100%;
      }

      &::after {
        content: '';
        position: absolute;
        left: 0;
        top: 0;
        right: 0;
        bottom: 0;
        height: 64px;
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
</style>
