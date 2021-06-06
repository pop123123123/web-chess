import { Action } from '@/common/Action';
import Game, { GameId } from '@/common/Game';
import axios from 'axios';

const API_PREFIX = '/api';
const apisator = axios.create({ baseURL: API_PREFIX });

async function getMessage(): Promise<string> {
  return (await apisator.get('/message')).data;
}

async function getGame(gameId: GameId): Promise<Game> {
  return new Game(gameId, (await apisator.get(`/game/${gameId}`)).data.history);
}

async function importGame(history: Action[]): Promise<GameId> {
  return (await apisator.post('/game', history)).data.id;
}

async function createGame(): Promise<GameId> {
  return importGame([]);
}

async function sendAction(gameId: GameId, action: Action): Promise<void> {
  await apisator.put(`/game/${gameId}/action`, action);
}

async function resetGame(gameId: GameId): Promise<void> {
  await apisator.patch(`/game/${gameId}`);
}

async function deleteLastAction(gameId: GameId): Promise<void> {
  await apisator.delete(`/game/${gameId}/last_action`);
}

export default {
  getMessage,
  getGame,
  importGame,
  createGame,
  sendAction,
  resetGame,
  deleteLastAction,
};
