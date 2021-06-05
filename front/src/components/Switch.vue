<template>
  <div class="switch" :class="`switch-${theme}`">
    <input type="checkbox" v-model="internalValue"/>
    <div class="toggle-outside">
      <div class="toggle-inside">
        <span>{{ offText }}</span>
        <span>{{ onText }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Switch',
  data: () => ({
    internalValue: 'false',
  }),
  props: {
    value: Boolean,
    theme: {
      type: String,
      default: 'default',
    },
    offText: {
      type: String,
      default: 'Off',
    },
    onText: {
      type: String,
      default: 'On',
    },
  },
  emits: ['update:value'],
  watch: {
    internalValue(value) {
      this.$emit('update:value', value);
    },
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';

@mixin buttonColoring($name) {
  & {
    background: nth(map-get(theme.$switch, $name), 1);

    .toggle-outside {
      background: nth(map-get(theme.$switch, $name), 2);
    }

    .toggle-inside {
      background: nth(map-get(theme.$switch, $name), 3);

      span {
        color: nth(map-get(theme.$switch, $name), 2);
      }
    }

    input:checked ~ .toggle-outside .toggle-inside {
      background: nth(map-get(theme.$switch, $name), 4);
    }
  }
}

$switch-width: 60px;
$p: 5px;

.switch {
  position: relative;
  min-width: $switch-width;
  height: 30px;
  font-size: 0.8em;
  border: 3px solid #00000066;
  border-radius: 8px;
  padding: $p;

  input {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    z-index: 2;
    cursor: pointer;
    width: 100%;
    margin: 0;
    opacity: 0;

    &:focus-visible ~ .toggle-outside {
      box-shadow: 0 0 1px 3px #2af;
    }

    &:hover ~ .toggle-outside {
      filter: brightness(1.1);
    }

    ~ .toggle-outside span {
      opacity: 1;
      color: grey;
    }

    ~ .toggle-outside span:last-of-type {
      opacity: 0;
    }

    &:checked {
      z-index: 1;

      ~ .toggle-outside span {
        opacity: 0;
      }

      ~ .toggle-outside span:last-of-type {
        opacity: 1;
      }
    }
  }

  span {
    position: relative;
    top: 50%;
    color: inherit;
    line-height: 0;
    display: block;
    margin: 0;
    text-align: center;
    font-size: 0.8em;
    font-weight: bold;
    text-transform: uppercase;
  }

  .toggle-outside {
    position: relative;
    height: 100%;
    width: 100%;
    border-radius: 4px;
    overflow: hidden;
  }

  .toggle-inside {
    border-radius: 4px;
    transition: all .25s;
    height: 100%;
    min-width: 50%;
    top: 0;
    left: 0;
    position: absolute;
    padding: 0 8px;
  }

  input:checked ~ .toggle-outside .toggle-inside {
    left: 100%;
    transform: translateX(-100%);
  }

  @each $class, $colors in theme.$switch {
    &.switch-#{$class} {
      @include buttonColoring($class);
    }
  }
}
</style>
