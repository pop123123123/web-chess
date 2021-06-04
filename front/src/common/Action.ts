import Cell from './Cell';

interface ActionInterface {
  from: {
    row: number,
    column: number,
  },
  to: {
    row: number,
    column: number,
  },
}

class Action {
  from: Cell;

  to: Cell;

  constructor(from: Cell, to: Cell) {
    this.from = from;
    this.to = to;
  }

  static fromActionInterface(ai: ActionInterface): Action {
    const from = new Cell(ai.from.row, ai.from.column);
    const to = new Cell(ai.to.row, ai.to.column);
    return new Action(from, to);
  }

  toAlgebraicNotation(): string {
    const fromName = this.from.toCellName();
    const toName = this.to.toCellName();
    return `${fromName}-${toName}`;
  }
}

export { Action, ActionInterface };
