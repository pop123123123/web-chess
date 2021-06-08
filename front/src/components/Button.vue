<template>
  <button><div>
    <font-awesome-icon class="icon" :icon="icon"></font-awesome-icon>
    <slot></slot>
  </div></button>
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
@use 'sass:color';
@use '../scss/theme';

@mixin buttonColoring($name) {
  $color-1: nth(map-get(theme.$colors, $name), 1);
  $color-2: nth(map-get(theme.$colors, $name), 2);
  $color-3: nth(map-get(theme.$colors, $name), 3);

  & {
    background: $color-1;

    > div {
      background: $color-2;
      color: $color-3;

      > .icon {
        border-color: $color-1;
        color: color.adjust($color-3, $alpha: -0.2);
      }
    }

    &:active {
      background: $color-2;

      > div {
        background: $color-1;

        > .icon {
          border-color: $color-2;
        }
      }
    }
  }
}

button {
  padding: 5px;
  border-radius: 8px;
  border: 3px solid #00000066;
  font-size: 0.7em;
  font-family: inherit;
  @include buttonColoring('default');

  &:disabled {
    filter: grayscale(0.5);
    pointer-events: none;
    opacity: 0.8;
  }

  > div {
    border-radius: 4px;
    padding: 0 8px 0 36px;
    text-align: center;
    font-weight: bold;
    text-transform: uppercase;
    white-space: nowrap;
    line-height: 28px;
    height: 28px;
    position: relative;

    > .icon {
      border-right-width: 1px;
      border-right-style: solid;
      padding: 0 0.5em;
      margin-right: 0.5em;
      position: absolute;
      left: 0;
      height: 100%;
      text-align: center;
      width: 18px;
    }
  }

  &:hover {
    filter: brightness(1.1);
  }
}

@each $class, $colors in theme.$colors {
  button.#{$class} {
    @include buttonColoring($class);
  }
}
</style>
