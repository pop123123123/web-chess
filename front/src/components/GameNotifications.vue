<template>
  <notifications
    class="game-notification-group"
    position="bottom center"
    classes="game-notification"
  />
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'GameNotifications',
});
</script>

<style lang="scss">
@use '../scss/theme';

@mixin notificationColoring($name) {
  & {
    background: nth(map-get(theme.$colors, $name), 1);

    .notification-title {
      background: nth(map-get(theme.$colors, $name), 2);
      color: nth(map-get(theme.$colors, $name), 3);
    }
  }
}

.game-notification-group {
  position: absolute;

  .vue-notification-wrapper {
    overflow: visible;
  }
}

.vue-notification-template.game-notification {
  margin: 0 5px 5px;
  padding: 5px;
  border-radius: 12px;
  border: 3px solid #00000066;
  @include notificationColoring('default');

  .notification-title {
    border-radius: 6px;
    padding: 5px;
    text-align: center;
    font-size: 0.9em;
  }

  .notification-content {
    padding: 10px 5px 5px;
    text-align: center;
    font-size: 0.8em;

    &:empty {
      display: none;
    }
  }

  @each $class, $colors in theme.$colors {
    &.#{$class} {
      @include notificationColoring($class);
    }
  }
}
</style>
