import { Store } from 'vuex';
import State from './store-state';

declare module '@vue/runtime-core' {
  // provide typings for `this.$store`
  export interface ComponentCustomProperties {
    $store: Store<State>
  }
}
