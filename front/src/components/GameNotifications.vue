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
  background: #44a4fc;

  .notification-title {
    background: #187fe7;
    border-radius: 6px;
    padding: 5px;
    text-align: center;
    font-size: 0.9em;
    color: #ffffffdd;
  }

  .notification-content {
    padding: 10px 5px 5px;
    text-align: center;
    font-size: 0.8em;

    &:empty {
      display: none;
    }
  }

  &.success {
    background: #42a85f;

    .notification-title {
      background: #68cd86;
    }
  }

  &.warn {
    background: #f48a06;

    .notification-title {
      background: #ffb648;
      color: #000000aa;
    }
  }

  &.error {
    background: #b82e24;

    .notification-title {
      background: #e54d42;
    }
  }
}
</style>
