import Action from './Action';

export type GameId = number;

export const EMPTY_BOARD = [
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
];

export const INITIAL_BOARD = [
  ['rl', 'nl', 'bl', 'ql', 'kl', 'bl', 'nl', 'rl'],
  ['pl', 'pl', 'pl', 'pl', 'pl', 'pl', 'pl', 'pl'],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['  ', '  ', '  ', '  ', '  ', '  ', '  ', '  '],
  ['pd', 'pd', 'pd', 'pd', 'pd', 'pd', 'pd', 'pd'],
  ['rd', 'nd', 'bd', 'qd', 'kd', 'bd', 'nd', 'rd'],
];

export default class Game {
    id: GameId;

    history: Action[];

    constructor(id: GameId, history: Action[]) {
      this.id = id;
      this.history = history;
    }

    getBoard(): Array<Array<string>> {
      // copy initial board
      const board = JSON.parse(JSON.stringify(INITIAL_BOARD));

      // apply actions
      this.history.forEach((action) => {
        const piece = board[action.from.row][action.from.column];
        board[action.to.row][action.to.column] = piece;
        board[action.from.row][action.from.column] = '  ';
      });

      return board;
    }
}
