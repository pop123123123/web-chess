import { Action, ActionInterface } from './Action';
import Piece, { PieceColor, PieceType } from './Piece';
import { BASE64_ENCODE } from './b64';

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

  lastPieceToMove: string;

  lastPieceToDie: Piece | undefined;

  constructor(id: GameId, history: ActionInterface[]) {
    this.id = id;
    this.history = history.map((ai) => Action.fromActionInterface(ai));
    this.lastPieceToMove = '';
    this.lastPieceToDie = undefined;
  }

  getBoard(): string[][] {
    // copy initial board
    const board = JSON.parse(JSON.stringify(INITIAL_BOARD)) as string[][];

    // apply actions
    this.history.forEach((action) => {
      const piece = board[action.from.row][action.from.column];
      const lastPieceToDieId = board[action.to.row][action.to.column];

      board[action.to.row][action.to.column] = piece;
      board[action.from.row][action.from.column] = '   ';

      this.lastPieceToMove = piece;
      this.lastPieceToDie = /^[bknpqr][dl]/.test(lastPieceToDieId) ? {
        id: lastPieceToDieId,
        type: lastPieceToDieId[0] as PieceType,
        color: lastPieceToDieId[1] as PieceColor,
        row: action.to.row,
        column: action.to.column,
        moving: false,
        dead: true,
      } : undefined;
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
            moving: square === this.lastPieceToMove,
            dead: false,
          });
        }
      });
    });
    if (this.lastPieceToDie !== undefined) {
      pieces.push(this.lastPieceToDie);
    }
    return pieces.sort((a, b) => a.id.localeCompare(b.id));
  }

  toBase64(): string {
    return this.history.flatMap((ac) => ac.asArray()).map((i) => BASE64_ENCODE[i]).join('');
  }
}
