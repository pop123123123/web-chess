<template>
  <div class="history" :class="{ expand: showMore }">
    <transition-group name="list-complete" tag="ul" class="history-list">
      <li v-for="(s, i) in history" :key="i" class="list-complete-item">
        {{i+1}}. {{s}}
      </li>
    </transition-group>
    <div class="show-more">
      <input type="checkbox" id="show-more" v-model="showMore">
      <label for="show-more">▽</label>
    </div>
  </div>
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
  data() {
    return {
      showMore: false,
    };
  },
});
</script>

<style lang="scss" scoped>
@use '../scss/theme';

$list-item-height: 3.5em;
$list-width: 12em;
$list-padding-mobile: 3em;
$list-expanded-height: $list-item-height * 3;

*, *:before, *:after {
  box-sizing: border-box;
}

.history {
  transition: height 0.5s;

  .show-more {
    display: none;
  }
}

.history-list {
  padding: 0;
  width: $list-width;

  .list-complete {
    &-item {
      transition: all 0.8s ease;
      display: inline-block;
      height: $list-item-height;
      margin: 0;
      padding: 20px 0 20px $list-width * 0.2;
      width: $list-width;
      text-align: start;
      border-bottom: 1px solid theme.$background-main;

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
  .history {
    display: flex;
    overflow: hidden;
    height: $list-item-height;
    align-items: flex-end;

    &.expand {
      height: $list-expanded-height;

      .history-list {
        overflow: auto;
        max-height: $list-expanded-height;
      }

      .show-more input[type="checkbox"] + label {
        transform: scaleY(-1);
      }
    }

    .history-list {
      overflow: hidden;
      flex: 1;
      align-items: flex-start;

      .list-complete-item {
        padding-left: $list-padding-mobile;
        width: 100%;

        &:last-child {
          border-bottom-color: transparent;
        }
      }
    }

    .show-more {
      display: block;

      input[type="checkbox"] {
        display: none;

        + label {
          display: block;
          font-size: $list-item-height / 2;
          line-height: 1em;
          padding: 0.5em;
          transition: transform 0.5s;
        }
      }
    }
  }
}
</style>
