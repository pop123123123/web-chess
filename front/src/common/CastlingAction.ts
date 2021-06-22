import { Action, ActionInterface, ActionRequestInterface } from './Action';
import Cell from './Cell';

export default class CastlingAction extends Action {
  tower_from: Cell;

  tower_to: Cell;

  constructor(from: Cell, to: Cell, tower_from: Cell, tower_to: Cell) {
    super(from, to);
    this.tower_from = tower_from;
    this.tower_to = tower_to;
  }

  static fromActionInterface(ai: ActionInterface): CastlingAction {
    const data = ai.Castling;
    const row = data.color === 'Black' ? 7 : 0;
    const from = new Cell(row, 4);
    const to = new Cell(row, data.side === 'King' ? 6 : 2);
    const towerFrom = new Cell(row, data.side === 'King' ? 7 : 0);
    const towerTo = new Cell(row, data.side === 'King' ? 5 : 3);
    return new CastlingAction(from, to, towerFrom, towerTo);
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to };
  }

  toAlgebraicNotation(): string {
    if (this.tower_from.column > this.from.column) {
      return '0-0';
    }
    return '0-0-0';
  }
}
