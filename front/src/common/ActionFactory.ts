import { Action, ActionInterface } from './Action';
import CastlingAction from './CastlingAction';
import Cell from './Cell';
import EnPassantAction from './EnPassantAction';
import PromotionAction from './PromotionAction';
import StandardAction from './StandardAction';

export default class ActionFactory {
  static fromActionInterface(ai: ActionInterface): Action {
    if ('Standard' in ai) {
      return StandardAction.fromActionInterface(ai);
    }
    if ('Promotion' in ai) {
      return PromotionAction.fromActionInterface(ai);
    }
    if ('Castling' in ai) {
      return CastlingAction.fromActionInterface(ai);
    }
    if ('EnPassant' in ai) {
      return EnPassantAction.fromActionInterface(ai);
    }
    throw new Error('not implemented');
  }

  static fromArray(a: number[]): Action {
    const [from, to] = a;
    return new StandardAction(Cell.fromUint(from), Cell.fromUint(to));
  }
}
