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

a {
  color: inherit;
}

.view {
  padding: 1em;
}
</style>
