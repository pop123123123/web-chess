<template>
  <div class="title"><span><slot></slot></span></div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Button',
  props: {
    icon: {
      type: [String, Array],
      default: undefined,
    },
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';

.title {
  position: relative;
  z-index: 1;
  padding: 0.5em 0;
  margin: 10px 0 5px;
  text-align: center;
  white-space: nowrap;

  &::before {
    content: '';
    display: block;
    margin: 0 auto;
    border-top: 1px solid theme.$background-main;
    position: absolute;
    top: 50%;
    right: -30px;
    left: -30px;
    z-index: -1;
  }

  span {
    background: theme.$background-secondary;
    padding: 0 0.5em;
  }
}
</style>
