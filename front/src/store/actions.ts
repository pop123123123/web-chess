import api from '@/api';
import { ActionTree } from 'vuex';
import { Actions, ActionTypes } from './action-types';
import { MutationTypes } from './mutation-types';
import { State } from './state';

const actions: ActionTree<State, State> & Actions = {
  async [ActionTypes.INIT_GAME]({ commit }, gameId) {
    commit(MutationTypes.SET_GAME, await api.getGame(gameId));
  },
};

export default actions;
