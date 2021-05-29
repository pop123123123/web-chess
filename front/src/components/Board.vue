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
      >{{ square }}</div>
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
});
</script>

<style scoped lang="scss">
.board {
  width: 256px;
  height: 256px;
  background: black;

  .row {
    &:nth-child(2n) {
      .square:nth-child(2n) {
        background: lightgray;
      }
    }

    &:nth-child(2n+1) {
      .square:nth-child(2n+1) {
        background: lightgray;
      }
    }

    .square {
      float: left;
      width: 32px;
      height: 32px;
      background: gray;
    }
  }
}
</style>
