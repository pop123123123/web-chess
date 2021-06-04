<template>
  <transition-group name="list-complete" tag="ul" class="history">
    <li v-for="(s, i) in history" :key="i" class="list-complete-item">
      {{i+1}}. {{s}}
    </li>
  </transition-group>
</template>

<script>
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'History',
  props: {
    history: [String],
  },
});
</script>

<style lang="scss" scoped>
.list-complete-item {
  transition: all 0.8s ease;
  display: inline-block;
  margin-right: 10px;
}

.list-complete-enter-from, .list-complete-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.list-complete-leave-active {
  position: absolute;
}

</style>
