import Action from './Action';
import Piece, { PieceColor, PieceType } from './Piece';

export type GameId = number;

export const INITIAL_BOARD = [
  ['rl0', 'nl0', 'bl0', 'ql0', 'kl0', 'bl1', 'nl1', 'rl1'],
  ['pl0', 'pl1', 'pl2', 'pl3', 'pl4', 'pl5', 'pl6', 'pl7'],
  ['   ', '   ', '   ', '   ', '   ', '   ', '   ', '   '],
  ['   ', '   ', '   ', '   ', '   ', '   ', '   ', '   '],
  ['   ', '   ', '   ', '   ', '   ', '   ', '   ', '   '],
  ['   ', '   ', '   ', '   ', '   ', '   ', '   ', '   '],
  ['pd0', 'pd1', 'pd2', 'pd3', 'pd4', 'pd5', 'pd6', 'pd7'],
  ['rd0', 'nd0', 'bd0', 'qd0', 'kd0', 'bd1', 'nd1', 'rd1'],
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
        board[action.from.row][action.from.column] = '   ';
      });

      return board;
    }

    getPieces(): Piece[] {
      const pieces = [] as Piece[];
      this.getBoard().forEach((row, rowIndex) => {
        row.forEach((square, colIndex) => {
          if (/^[bknpqr][dl]/.test(square)) {
            pieces.push({
              id: square,
              type: square[0] as PieceType,
              color: square[1] as PieceColor,
              row: rowIndex,
              column: colIndex,
            });
          }
        });
      });
      return pieces.sort((a, b) => a.id.localeCompare(b.id));
    }
}
