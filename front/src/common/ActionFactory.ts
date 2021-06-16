import { Action, ActionInterface } from './Action';
import Cell from './Cell';
import StandardAction from './StandardAction';

export default class ActionFactory {
  static fromActionInterface(ai: ActionInterface): Action {
    if ('Standard' in ai) {
      const data = ai.Standard;
      const from = new Cell(data.from.row, data.from.column);
      const to = new Cell(data.to.row, data.to.column);
      return new StandardAction(from, to);
    }
    throw new Error('not implemented');
  }

  static fromArray(a: number[]): Action {
    const [from, to] = a;
    return new StandardAction(Cell.fromUint(from), Cell.fromUint(to));
  }
}
