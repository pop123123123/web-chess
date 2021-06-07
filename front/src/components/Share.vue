<template>
  <Button class="info" @click="trySharingElseCopy">{{ buttonTitle }}</Button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Button from '@/components/Button.vue';

export default defineComponent({
  name: 'Share',
  components: {
    Button,
  },
  props: {
    buttonTitle: {
      type: String,
      default: 'Share',
    },
    title: {
      type: String,
      required: true,
    },
    url: {
      type: String,
      default: undefined,
    },
    text: {
      type: String,
      default: undefined,
    },
  },
  data: () => ({
    copied: false,
  }),
  methods: {
    async copy() {
      await navigator.clipboard.writeText(this.url || this.text);
    },
    async share() {
      const data = { title: this.title, url: this.url, text: this.text };
      await navigator.share(data);
    },
    async trySharingElseCopy() {
      try {
        await this.share();
      } catch (err) {
        try {
          await this.copy();
          this.$notify({
            title: 'Copied to clipboard!',
            type: 'info',
          });
        } catch (_err) {
          // eslint-disable-next-line no-alert
          window.prompt('Could not copy, please copy manually.', this.url || this.text);
        }
      }
    },
  },

});
</script>
