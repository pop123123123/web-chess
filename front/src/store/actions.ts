import api from '@/api';
import { ActionTree } from 'vuex';
import { Actions } from './action-types';
import { State } from './state';

const actions: ActionTree<State, State> & Actions = {
  CREATE_GAME() {
    return api.createGame();
  },
  IMPORT_GAME(_, history) {
    return api.importGame(history);
  },
  async GET_GAME({ commit }, gameId) {
    commit('SET_GAME', await api.getGame(gameId));
  },
  RESET_GAME(_, gameId) {
    return api.resetGame(gameId);
  },
  SEND_ACTION(_, { gameId, action }) {
    return api.sendAction(gameId, action);
  },
  TRY_ACTION(_, { gameId, action }) {
    return api.tryAction(gameId, action);
  },
  DELETE_LAST_ACTION(_, gameId) {
    return api.deleteLastAction(gameId);
  },
  async CLEAR_GAME_STATE({ commit }) {
    commit('SET_GAME', undefined);
  },
};

export default actions;
