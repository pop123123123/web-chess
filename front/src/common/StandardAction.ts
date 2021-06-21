import { Action, ActionInterface, ActionRequestInterface } from './Action';
import Cell from './Cell';

export default class StandardAction extends Action {
  static fromActionInterface(ai: ActionInterface): StandardAction {
    const data = ai.Standard;
    const from = new Cell(data.from.row, data.from.column);
    const to = new Cell(data.to.row, data.to.column);
    return new StandardAction(from, to);
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to };
  }
}
