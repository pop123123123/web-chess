import { MutationTree } from 'vuex';
import { Mutations } from './mutation-types';
import { State } from './state';

const mutations: MutationTree<State> & Mutations = {
  SET_GAME(state, game) {
    state.game = game;
  },
  SET_PROMOTION(state, promotion) {
    state.promotion = promotion;
  },
};

export default mutations;
