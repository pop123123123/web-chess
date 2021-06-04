<template>
  <div class="switch" :class="`switch-${theme}`">
    <input id="a" type="radio" value="false" v-model="internalValue"/>
    <label for="a">{{ offText }}</label>
    <input id="b" type="radio" value="true" v-model="internalValue"/>
    <label for="b">{{ onText }}</label>
    <span class="toggle-outside">
      <span class="toggle-inside"></span>
    </span>
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
      this.$emit('update:value', value === 'true');
    },
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';

@mixin buttonColoring($name) {
  & {
    .toggle-outside {
      background: nth(map-get(theme.$switch, $name), 1);
    }

    .toggle-inside {
      background: nth(map-get(theme.$switch, $name), 2);
    }

    input ~ input:checked ~ .toggle-outside .toggle-inside {
      background: nth(map-get(theme.$switch, $name), 3);
    }
  }
}

*, *:before, *:after {
  box-sizing: border-box;
}

.switch {
  position: relative;
  width: 200px;
  height: 40px;
  font-size: 0.9em;

  input {
    position: absolute;
    top: 0;
    z-index: 2;
    opacity: 0;
    cursor: pointer;
    height: 40px;
    width: 80px;
    left: 60px;
    margin: 0;

    &:hover ~ .toggle-outside {
      filter: brightness(1.1);
    }

    &:checked {
      z-index: 1;

      & + label {
        opacity: 1;
        cursor: default;
      }
    }

    &:not(:checked) + label:hover {
      opacity: .5;
    }
  }

  label {
    color: inherit;
    opacity: .33;
    cursor: pointer;
    line-height: 40px;
    display: inline-block;
    width: 60px;
    height: 100%;
    margin: 0;
    text-align: center;
  }

  label:last-of-type {
    margin-left: 80px;
  }

  .toggle-outside {
    height: 100%;
    border-radius: 8px;
    padding: 5px;
    overflow: hidden;
    position: absolute;
    width: 80px;
    left: 60px;
    border: 3px solid #00000066;
  }

  .toggle-inside {
    border-radius: 4px;
    position: absolute;
    transition: left .25s;
    height: 24px;
    width: 35px;
  }

  input:checked ~ .toggle-outside .toggle-inside {
    left: 5px;
  }

  input ~ input:checked ~ .toggle-outside .toggle-inside {
    left: 34px;
  }

  @each $class, $colors in theme.$switch {
    &.switch-#{$class} {
      @include buttonColoring($class);
    }
  }
}
</style>
