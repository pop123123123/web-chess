import Cell from './Cell';
import { PieceType } from './Piece';

interface ActionRequestInterface {
  from: {
    row: number,
    column: number,
  },
  to: {
    row: number,
    column: number,
  },
  piece?: PieceType,
}

interface ActionInterface {
  Standard: {
    from: {
      row: number,
      column: number,
    },
    to: {
      row: number,
      column: number,
    },
  }
}

abstract class Action {
  from: Cell;

  to: Cell;

  constructor(from: Cell, to: Cell) {
    this.from = from;
    this.to = to;
  }

  toAlgebraicNotation(): string {
    const fromName = this.from.toCellName();
    const toName = this.to.toCellName();
    return `${fromName}-${toName}`;
  }

  asArray(): number[] {
    return [this.from.asUint(), this.to.asUint()];
  }

  abstract serialize(): ActionRequestInterface;
}

export { Action, ActionInterface, ActionRequestInterface };
