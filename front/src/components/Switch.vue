<template>
  <div class="switch">
    <input id="a" type="radio" value="true" v-model="value"/>
    <label for="a">White</label>
    <input id="b" type="radio" value="false" v-model="value"/>
    <label for="b">Black</label>
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
    value: true,
  }),
  props: {
    modelValue: {
      type: Boolean,
      default: true,
    },
  },
  emits: ['update:modelValue'],
  methods: {
    changePageTitle(value: boolean) {
      this.$emit('update:modelValue', value);
    },
  },
  // TODO watch for changes, link value / modelValue
});
</script>

<style lang="scss" scoped>
$text-color: white;
$primary-color: white;
$secondary-color: #222;

*, *:before, *:after {
  box-sizing: border-box;
}

.switch {
  position: relative;
  width: 18rem;
  height: 3rem;
  margin: 0 auto;
  font-size: 0;
  margin-bottom: 1rem;
  & input {
    position: absolute;
    top: 0;
    z-index: 2;
    opacity: 0;
    cursor: pointer;
    height: 3rem;
    width: 6rem;
    left: 6rem;
    margin: 0;
    &:checked   {
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
  & label {
    color: $text-color;
    opacity: .33;
    transition: opacity .25s ease;
    cursor: pointer;
    font-size: 1.5rem;
    line-height: 3rem;
    display: inline-block;
    width: 6rem;
    height: 100%;
    margin: 0;
    text-align: center;
  }
  & label:last-of-type  {
    margin-left: 6rem;
  }
  & .toggle-outside {
    height: 100%;
    border-radius: 2rem;
    padding: .25rem;
    overflow: hidden;
    transition: .25s ease all;
    background: $secondary-color;
    position: absolute;
    width: 6rem;
    left: 6rem;
  }
  & input ~ input:checked ~ .toggle-outside {
    background: $primary-color;
  }
  & .toggle-inside {
    border-radius: 5rem;
    background: $primary-color;
    position: absolute;
    transition: .25s ease all;
    height: 2.5rem;
    width: 2.5rem;
  }
  & input:checked ~ .toggle-outside .toggle-inside {
    left: .25rem;
  }
  & input ~ input:checked ~ .toggle-outside .toggle-inside {
    left: 3.25rem;
    background: $secondary-color;
  }
  & input:checked + label:hover ~ .toggle-outside .toggle-inside {
      height: 2.5rem;
      width: 2.5rem;
  }
  & input:hover ~ .toggle-outside .toggle-inside   {
    width: 3.5rem;
  }
  & input:hover ~ input:checked ~ .toggle-outside .toggle-inside {
    left: 2.25rem;
  }
}
</style>
