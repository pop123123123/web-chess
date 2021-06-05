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
