import { Store } from '@/store';

declare module '@vue/runtime-core' {
  // provide typings for `this.$store`
  export interface ComponentCustomProperties {
    $store: Store
  }
}
