import { MutationTree } from 'vuex';
import { Mutations, MutationTypes } from './mutation-types';
import { State } from './state';

const mutations: MutationTree<State> & Mutations = {
  [MutationTypes.SET_GAME](state, game) {
    state.game = game;
  },
};

export default mutations;
