import Action from '@/common/Action';
import Game, { GameId } from '@/common/Game';
import axios from 'axios';

export default {
  async getMessage(): Promise<string> {
    return (await axios.get('/message')).data;
  },

  async getGame(gameId: GameId): Promise<Game> {
    return (await axios.get(`/game/${gameId}`)).data;
  },

  async createGame(): Promise<GameId> {
    return (await axios.post('/game')).data;
  },

  async sendAction(gameId: GameId, action: Action): Promise<void> {
    await axios.put(`/game/${gameId}/action`, action);
  },
};
