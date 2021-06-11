import { Action } from '@/common/Action';
import { ActionContext } from 'vuex';
import { Mutations } from './mutation-types';
import { State } from './state';

// Augmented ActionContext
type AAC = {
  commit<K extends keyof Mutations>(
    key: K,
    payload: Parameters<Mutations[K]>[1]
  ): ReturnType<Mutations[K]>
} & Omit<ActionContext<State, State>, 'commit'>;

export interface Actions {
  CREATE_GAME(ctx: AAC): Promise<number>
  IMPORT_GAME(ctx: AAC, history: Action[]): Promise<number>
  GET_GAME(ctx: AAC, gameId: number): Promise<void>
  RESET_GAME(ctx: AAC, gameId: number): Promise<void>
  SEND_ACTION(ctx: AAC, payload: { gameId: number, action: Action }): Promise<void>
  DELETE_LAST_ACTION(ctx: AAC, gameId: number): Promise<void>
}

export type ActionTypes = keyof Actions;
