<template>
  <div class="promotion">
    <TitleSeparator class="board-overlay-title">Promotion</TitleSeparator>
    <div class="promotion-pieces">
      <img :src="getPieceImage('nl')" alt="nl" @click="choosePiece('n')">
      <img :src="getPieceImage('rl')" alt="rl" @click="choosePiece('r')">
      <img :src="getPieceImage('bl')" alt="bl" @click="choosePiece('b')">
      <img :src="getPieceImage('ql')" alt="ql" @click="choosePiece('q')">
    </div>
    <Button class="warn" icon="times" @click="cancel()">Cancel move</Button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Button from '@/components/Button.vue';
import TitleSeparator from '@/components/TitleSeparator.vue';
import { getPieceImage, PieceType } from '@/common/Piece';

export default defineComponent({
  name: 'PromotionPrompt',
  components: {
    Button,
    TitleSeparator,
  },
  emits: {
    choosePiece: (name: PieceType) => (name.match(/^[nrbq]$/)),
    cancel: null,
  },
  methods: {
    getPieceImage(name: string) {
      return getPieceImage(name);
    },
    choosePiece(name: PieceType) {
      this.$emit('choosePiece', name);
    },
    cancel() {
      this.$emit('cancel');
    },
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';

.promotion {
  background: theme.$board-square-dark;
  border: 3px solid theme.$board-border-color;
  width: 50%;
  max-height: 50%;
  padding: 10px 20px 20px;
  border-radius: 8px;
  overflow: hidden auto;
  text-align: center;

  .promotion-pieces {
    margin-bottom: 10px;

    img {
      width: 25%;
      cursor: pointer;
      border-radius: 8px;

      &:hover {
        background: theme.$board-square-light;
      }
    }
  }
}
</style>
