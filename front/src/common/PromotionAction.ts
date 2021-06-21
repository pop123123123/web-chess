import { Action, ActionInterface, ActionRequestInterface } from './Action';
import Cell from './Cell';
import { PieceType } from './Piece';

export default class PromotionAction extends Action {
  piece: PieceType;

  constructor(from: Cell, to: Cell, piece: PieceType) {
    super(from, to);
    this.piece = piece;
  }

  static fromActionInterface(ai: ActionInterface): PromotionAction {
    const data = ai.Promotion;
    const from = new Cell(data.color === 'Black' ? 1 : 6, data.start_column);
    const directionOffsets = {
      Straight: 0,
      Left: -1,
      Right: 1,
    };
    const to = new Cell(data.color === 'Black' ? 0 : 7, data.start_column + directionOffsets[data.direction]);
    const pieces = {
      Knight: PieceType.Knight,
      Rook: PieceType.Rook,
      Bishop: PieceType.Bishop,
      Queen: PieceType.Queen,
    };
    return new PromotionAction(from, to, pieces[data.promote_piece]);
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to, piece: this.piece };
  }
}
