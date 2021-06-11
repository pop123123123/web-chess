import { createStore } from 'vuex';
import State from '@/typings/store-state';
import Game from '@/common/Game';
import api from '@/api';

export default createStore<State>({
  state: {
    game: undefined,
  },
  mutations: {
    setGame(state, game: Game) {
      state.game = game;
    },
  },
  actions: {
    async initGame(context, gameId: number) {
      context.commit('setGame', await api.getGame(gameId));
    },
  },
  modules: {
  },
});
