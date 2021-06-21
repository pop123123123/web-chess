import { Action, ActionInterface, ActionRequestInterface } from './Action';
import Cell from './Cell';

export default class EnPassantAction extends Action {
  static fromActionInterface(ai: ActionInterface): EnPassantAction {
    const data = ai.EnPassant;
    const from = new Cell(data.from.row, data.from.column);
    const to = new Cell(data.to.row, data.to.column);
    return new EnPassantAction(from, to);
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to };
  }

  getTargetCell(): Cell {
    return new Cell(this.from.row, this.to.column);
  }
}
