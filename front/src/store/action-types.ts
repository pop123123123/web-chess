import { ActionContext } from 'vuex';
import { Mutations } from './mutation-types';
import { State } from './state';

export enum ActionTypes {
  INIT_GAME = 'INIT_GAME',
}

type AugmentedActionContext = {
  commit<K extends keyof Mutations>(
    key: K,
    payload: Parameters<Mutations[K]>[1]
  ): ReturnType<Mutations[K]>
} & Omit<ActionContext<State, State>, 'commit'>;

export interface Actions {
  [ActionTypes.INIT_GAME](
    { commit }: AugmentedActionContext,
    payload: number
  ): Promise<void>
}
