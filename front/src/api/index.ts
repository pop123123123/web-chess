import Action from '@/common/Action';
import Game, { GameId } from '@/common/Game';
import axios from 'axios';

const API_PREFIX = '/api';
const apisator = axios.create({ baseURL: API_PREFIX });

export default {
  async getMessage(): Promise<string> {
    return (await apisator.get('/message')).data;
  },

  async getGame(gameId: GameId): Promise<Game> {
    return new Game(gameId, (await apisator.get(`/game/${gameId}`)).data.history);
  },

  async createGame(): Promise<GameId> {
    return (await apisator.post('/game')).data.id;
  },

  async sendAction(gameId: GameId, action: Action): Promise<void> {
    await apisator.put(`/game/${gameId}/action`, action);
  },
};
