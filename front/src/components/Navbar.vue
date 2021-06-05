<template>
  <div id="nav">
    <router-link v-for="([name, path], i) in routes"
      :key="i" :to="path" :class="selected === name ? 'selected': ''">
      {{name}}
    </router-link>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Navbar',
  data: () => ({
    routes: [
      ['Home', '/'],
      ['About', '/about'],
    ],
  }),
  computed: {
    selected(): string {
      return this.$router.currentRoute.value.name?.toString() ?? '';
    },
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';
$hover-color: #556b2f44;

#nav {
  background: theme.$background-header;
  color: theme.$color-text-dark;
  padding: 1rem;
  text-align: center;
  height: 1.5rem;
  font-size: 1.25rem;

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
</style>
