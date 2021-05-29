<template>
  <div class="board">
    <div
      class="row"
      v-for="(row, index) in rows"
      :key="index"
    >
      <div
        class="square"
        v-for="(square, index) in row"
        :key="index"
      ><img :src="getSquareImage(square)" :alt="square"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';

export default defineComponent({
  name: 'Board',
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
      background: peru;
      overflow: hidden;

      img {
        width: 100%;
      }
    }
  }
}
</style>
