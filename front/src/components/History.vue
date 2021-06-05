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
    history: {
      type: Array,
      default: () => [],
    },
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';

.history {
  padding: 0;
  width: 12em;

  .list-complete {
    &-item {
      transition: all 0.8s ease;
      display: inline-block;
      margin-right: 10px;
      border-bottom: 1px solid theme.$background-main;
      padding: 20px 20%;
      padding-right: 0;
      margin: 0;
      width: 80%;
      text-align: start;

      &:first-child {
        border-top: 1px solid theme.$background-main;
      }
    }

    &-enter-from, &-leave-to {
      opacity: 0;
      transform: translateY(30px);
    }

    &-leave-active {
      position: absolute;
    }
  }
}

@media screen and (max-width: 900px) {

}
</style>
