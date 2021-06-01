import Action from './Action';
import Piece, { PieceColor, PieceType } from './Piece';

export type GameId = number;

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

    getBoard(): string[][] {
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

    getPieces(): Piece[] {
      const pieces = [] as Piece[];
      this.getBoard().forEach((row, rowIndex) => {
        row.forEach((square, colIndex) => {
          if (square.match(/[bknpqr][dl]/)) {
            pieces.push({
              type: square[0] as PieceType,
              color: square[1] as PieceColor,
              row: rowIndex,
              column: colIndex,
            });
          }
        });
      });
      return pieces;
    }
}
