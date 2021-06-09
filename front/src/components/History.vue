<template>
  <div class="history" :class="{ expand: showMore }">
    <div ref="wrapper" class="history-list-wrapper">
      <transition-group ref="list" name="list-complete" tag="ul" class="history-list">
        <li v-for="(s, i) in history" :key="i" :ref="setItemRef" class="list-complete-item">
          {{i+1}}. {{s}}
        </li>
      </transition-group>
      <div class="show-more">
        <input type="checkbox" id="show-more" v-model="showMore">
        <label for="show-more">▽</label>
      </div>
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
      itemRefs: [],
      lastLength: 0,
    };
  },
  methods: {
    setItemRef(el) {
      if (el) {
        this.itemRefs.push(el);
      }
    },
    scrollToBottom() {
      const el = this.$refs.wrapper;
      el.scrollTop = el.scrollHeight;

      const el2 = this.$refs.list.$el;
      el2.scrollTop = el2.scrollHeight;
    },
  },
  watch: {
    showMore() {
      this.scrollToBottom();
    },
  },
  beforeUpdate() {
    this.lastLength = this.itemRefs.length;
    this.itemRefs = [];
  },
  updated() {
    if (this.itemRefs.length !== this.lastLength) {
      this.scrollToBottom();
    }
  },
  mounted() {
    this.scrollToBottom();
  },
});
</script>

<style lang="scss" scoped>
@use 'sass:math';
@use '../scss/theme';

$list-item-height: 3.5em;
$list-width: 12em;
$list-padding-mobile: 3em;
$list-expanded-height: $list-item-height * 3.5;

*, *:before, *:after {
  box-sizing: border-box;
}

.history {
  position: relative;
  flex: 1;
  width: $list-width;
  transition: height 0.5s;

  .show-more {
    display: none;
  }
}

.history-list-wrapper {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  overflow: hidden auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.history-list {
  width: $list-width;
  min-height: 0;
  margin: 0;
  padding: 0;
  list-style: none;

  .list-complete {
    &-item {
      transition: opacity 0.8s ease, transform 0.8s ease;
      display: inline-block;
      height: $list-item-height;
      margin: 0;
      padding: 20px 0 20px $list-width * 0.2;
      width: $list-width;
      text-align: start;
      border-bottom: 1px solid theme.$background-main;
      left: 0;

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
    flex: unset;
    width: auto;
    height: $list-item-height;

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

    .history-list-wrapper {
      position: static;
      flex: 1;
      align-items: flex-end;
      flex-direction: row;
    }

    .history-list {
      width: 100%;
      overflow: hidden;

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
          font-size: math.div($list-item-height, 2);
          line-height: 1em;
          padding: 0.5em;
          transition: transform 0.5s;
        }
      }
    }
  }
}
</style>
