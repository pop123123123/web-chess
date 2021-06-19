import { Action, ActionRequestInterface } from './Action';
import Cell from './Cell';
import { PieceType } from './Piece';

export default class PromotionAction extends Action {
  piece: PieceType;

  constructor(from: Cell, to: Cell, piece: PieceType) {
    super(from, to);
    this.piece = piece;
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to, piece: this.piece };
  }
}
