import Cell from './Cell';
import { PieceType } from './Piece';

interface ActionRequestInterface {
  from: Cell,
  to: Cell,
  piece?: PieceType,
}

type ActionInterface = {
  Standard: {
    from: Cell,
    to: Cell,
  }
} & {
  Promotion: {
    start_column: number,
    color: 'Black' | 'White',
    direction: 'Straight' | 'Left' | 'Right',
    promote_piece: 'Knight' | 'Rook' | 'Bishop' | 'Queen',
  }
} & {
  Castling: {
    side: 'King' | 'Queen',
    color: 'Black' | 'White',
  }
} & {
  EnPassant: {
    from: Cell,
    to: Cell,
  }
};

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
    return `${fromName}${toName}`;
  }

  asArray(): number[] {
    return [this.from.asUint(), this.to.asUint()];
  }

  abstract serialize(): ActionRequestInterface;
}

export { Action, ActionInterface, ActionRequestInterface };
