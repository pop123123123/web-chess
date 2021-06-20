import { Action, ActionRequestInterface } from './Action';
import Cell from './Cell';

export default class CastlingAction extends Action {
  tower_from: Cell;

  tower_to: Cell;

  constructor(from: Cell, to: Cell, tower_from: Cell, tower_to: Cell) {
    super(from, to);
    this.tower_from = tower_from;
    this.tower_to = tower_to;
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to };
  }
}
