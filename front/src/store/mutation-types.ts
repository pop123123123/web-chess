import Game from '@/common/Game';
import { State } from './state';

export enum MutationTypes {
  SET_GAME = 'SET_GAME',
}

export type Mutations<S = State> = {
  [MutationTypes.SET_GAME](state: S, game: Game): void
};
