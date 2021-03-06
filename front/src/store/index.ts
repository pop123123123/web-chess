import {
  CommitOptions, createStore, DispatchOptions, Store as VuexStore,
} from 'vuex';
import { State, state } from './state';
import { Getters, getters } from './getters';
import mutations from './mutations';
import actions from './actions';
import { Actions } from './action-types';
import { Mutations } from './mutation-types';

export default createStore<State>({
  state,
  getters,
  mutations,
  actions,
});

export type Store = Omit<
VuexStore<State>,
'getters' | 'commit' | 'dispatch'
> & {
  commit<K extends keyof Mutations, P extends Parameters<Mutations[K]>[1]>(
    key: K,
    payload: P,
    options?: CommitOptions
  ): ReturnType<Mutations[K]>
} & {
  dispatch<K extends keyof Actions>(
    key: K,
    payload?: Parameters<Actions[K]>[1],
    options?: DispatchOptions
  ): ReturnType<Actions[K]>
} & {
  getters: {
    [K in keyof Getters]: ReturnType<Getters[K]>
  }
};
