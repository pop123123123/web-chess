import { Action, ActionInterface } from './Action';
import Cell from './Cell';
import { PieceType } from './Piece';
import PromotionAction from './PromotionAction';
import StandardAction from './StandardAction';

export default class ActionFactory {
  static fromActionInterface(ai: ActionInterface): Action {
    if (ai.Standard) {
      const data = ai.Standard;
      const from = new Cell(data.from.row, data.from.column);
      const to = new Cell(data.to.row, data.to.column);
      return new StandardAction(from, to);
    }
    if (ai.Promotion) {
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
    throw new Error('not implemented');
  }

  static fromArray(a: number[]): Action {
    const [from, to] = a;
    return new StandardAction(Cell.fromUint(from), Cell.fromUint(to));
  }
}
