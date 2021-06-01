<template>
<div :class="{copied: copied}">
  <input type="text" readonly @click="copy" :value="text"/>
</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Copy',
  props: {
    text: {
      type: String,
      required: true,
    },
  },
  data: () => ({
    copied: false,
  }),
  methods: {
    async copy() {
      await navigator.clipboard.writeText(this.text);
      this.copied = true;
      setTimeout(this.hideMessage, 1000);
    },
    hideMessage() {
      this.copied = false;
    },
  },

});
</script>

<style lang="scss" scoped>
@use 'sass:math';

div {
  position: relative;
  display: inline-block;
  input {
    cursor: pointer;
  }
  &::after {
    $h: 2em;
    content: 'Copied!';
    position: absolute;
    top: -50%;
    left: 50%;
    padding: .7em;
    height: $h;
    line-height: $h;
    transform: translate(-50%, math.div($h, 2));
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
