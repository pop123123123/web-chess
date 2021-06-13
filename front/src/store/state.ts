import Game from '@/common/Game';
import Promotion from '@/common/Promotion';

export const state = {
  game: undefined as Game | undefined,
  promotion: undefined as Promotion | undefined,
};

export type State = typeof state;
