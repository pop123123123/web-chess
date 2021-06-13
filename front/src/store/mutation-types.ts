import Game from '@/common/Game';
import Promotion from '@/common/Promotion';
import { State } from './state';

export type Mutations<S = State> = {
  SET_GAME(state: S, game: Game | undefined): void
  SET_PROMOTION(state: S, promotion: Promotion | undefined): void
};

export type MurationTypes = keyof Mutations;
