import api from '@/api';
import { ActionTree } from 'vuex';
import { Actions, ActionTypes } from './action-types';
import { MutationTypes } from './mutation-types';
import { State } from './state';

const actions: ActionTree<State, State> & Actions = {
  [ActionTypes.CREATE_GAME]() {
    return api.createGame();
  },
  [ActionTypes.IMPORT_GAME](_, history) {
    return api.importGame(history);
  },
  async [ActionTypes.GET_GAME]({ commit }, gameId) {
    commit(MutationTypes.SET_GAME, await api.getGame(gameId));
  },
  [ActionTypes.RESET_GAME](_, gameId) {
    return api.resetGame(gameId);
  },
  [ActionTypes.SEND_ACTION](_, { gameId, action }) {
    return api.sendAction(gameId, action);
  },
  [ActionTypes.DELETE_LAST_ACTION](_, gameId) {
    return api.deleteLastAction(gameId);
  },
};

export default actions;
