<template>
  <Button class="info" @click="trySharingElseCopy">Share</Button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Button from '@/components/Button.vue';

export default defineComponent({
  name: 'Share',
  components: {
    Button,
  },
  props: {
    title: {
      type: String,
      required: true,
    },
    url: {
      type: String,
      required: true,
    },
  },
  data: () => ({
    copied: false,
  }),
  methods: {
    async copy() {
      await navigator.clipboard.writeText(this.url);
    },
    async share() {
      const data = { title: this.title, url: this.url };
      await navigator.share(data);
    },
    async trySharingElseCopy() {
      try {
        await this.share();
        this.$notify({
          title: 'shared !',
          type: 'info',
        });
      } catch (err) {
        try {
          await this.copy();
          this.$notify({
            title: 'URL copied !',
            type: 'info',
          });
        } catch (_err) {
          this.$notify({
            title: 'Could not copy, please copy your browser\'s url',
            type: 'error',
          });
        }
      }
    },
    hideMessage() {
      this.copied = false;
    },
  },

});
</script>

<style lang="scss" scoped>
div {
  position: relative;
  display: inline-block;
  input {
    cursor: pointer;
  }
  &::after {
    content: 'Copied!';
    position: absolute;
    top: 50%;
    left: 50%;
    padding: 4px 10px;
    transform: translate(-50%, -50%);
    background: gray;
    border: 2px solid darkgray;
    opacity: 0;
    pointer-events : none;
    transition: 2s;
  }
  &.copied::after {
    display: block;
    opacity: 1;
    transition: .2s;
  }
}
</style>
