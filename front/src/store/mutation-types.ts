import Game from '@/common/Game';
import { State } from './state';

export type Mutations<S = State> = {
  SET_GAME(state: S, game: Game | undefined): void
};

export type MurationTypes = keyof Mutations;
