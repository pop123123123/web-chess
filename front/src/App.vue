<template>
  <Navbar/>
  <router-view/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  components: {
    Navbar,
  },
});
</script>

<style lang="scss">
@use './scss/theme';
$hover-color: #556b2f44;

body {
  display: flex;
  background: theme.$background-main;
  margin: 0;
  min-height: 100vh;
}

#app {
  display: flex;
  flex-direction: column;
  flex: 1;
  margin: 0;
  color: theme.$color-text;
  font-family: sans-serif;
}

#nav {
  background: theme.$background-header;
  color: theme.$color-text-dark;
  padding: 1em;
  text-align: center;

  a {
    text-decoration: none;
    padding: .5em 1.5em;
    margin: .5em .5em;
    text-transform: lowercase;
    font-variant: small-caps;
    transition: 0.2s;

    &:hover, &.selected {
      box-shadow: 0 0 1em 0px $hover-color, 0 0 .25em 0px $hover-color inset;
      background: #fff3;
    }

    &.selected {
      cursor: default;
    }
  }
}

a {
  color: inherit;
}

.view {
  padding: 1em;
}
</style>
