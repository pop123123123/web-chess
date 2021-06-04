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
$theme: (
  "default" : (#44a4fc, #187fe7, #ffffffdd),
  "success" : (#42a85f, #68cd86, #ffffffdd),
  "warn" : (#f48a06, #ffb648, #000000aa),
  "error" : (#b82e24, #e54d42, #ffffffdd),
);

@mixin notificationColoring($name) {
  & {
    background: nth(map-get($theme, $name), 1);

    .notification-title {
      background: nth(map-get($theme, $name), 2);
      color: nth(map-get($theme, $name), 3);
    }
  }
}

.game-notification-group {
  position: absolute;

  .vue-notification-wrapper {
    overflow: visible;
  }
}

.game-notification {
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

  @each $class, $colors in $theme {
    &.#{$class} {
      @include notificationColoring($class);
    }
  }
}
</style>
