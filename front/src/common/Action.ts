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

  asArray(): number[] {
    return [this.from.asUint(), this.to.asUint()];
  }

  static fromArray(a: number[]): Action {
    const [from, to] = a;
    return new Action(Cell.fromUint(from), Cell.fromUint(to));
  }
}

export { Action, ActionInterface };
