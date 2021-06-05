<template>
  <Navbar/>
  <router-view v-slot="{ Component }">
    <transition name="fade">
      <component :is="Component" />
    </transition>
  </router-view>
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

.fade-enter-active, .fade-leave-active, .fade-enter-from, .fade-leave-to {
  position: absolute;
  left: 0;
  right: 0;
  top: 3.5rem;
  bottom: 0;
}
.fade-enter-active {
  transition-delay: .1s !important;
}
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.25s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

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
