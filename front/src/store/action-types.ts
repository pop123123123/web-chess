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

export enum ActionTypes {
  CREATE_GAME = 'CREATE_GAME',
  IMPORT_GAME = 'IMPORT_GAME',
  GET_GAME = 'GET_GAME',
  RESET_GAME = 'RESET_GAME',
  SEND_ACTION = 'SEND_ACTION',
  DELETE_LAST_ACTION = 'DELETE_LAST_ACTION',
}

export interface Actions {
  [ActionTypes.CREATE_GAME](ctx: AAC): Promise<number>
  [ActionTypes.IMPORT_GAME](ctx: AAC, history: Action[]): Promise<number>
  [ActionTypes.GET_GAME](ctx: AAC, gameId: number): Promise<void>
  [ActionTypes.RESET_GAME](ctx: AAC, gameId: number): Promise<void>
  [ActionTypes.SEND_ACTION](ctx: AAC, payload: { gameId: number, action: Action }): Promise<void>
  [ActionTypes.DELETE_LAST_ACTION](ctx: AAC, gameId: number): Promise<void>
}
