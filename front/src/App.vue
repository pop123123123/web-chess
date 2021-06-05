<template>
  <div class="wrapper">
    <Navbar/>
    <router-view v-slot="{ Component }">
      <transition name="slide">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
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

.wrapper {
  width: 100vw;
  min-height: 100vh;
  overflow: hidden;
  position: relative;
}
.slide-enter-active,
.slide-leave-active {
  transition: all .75s ease-out;
  width: 100vw;
}

.slide-enter-to {
  position: absolute;
  left: 0;
}

.slide-enter-from {
  position: absolute;
  left: 100%;
}

.slide-leave-to {
  position: absolute;
  left: -100%;
}

.slide-leave-from {
  position: absolute;
  left: 0;
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
