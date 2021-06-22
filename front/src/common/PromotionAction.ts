import { Action, ActionInterface, ActionRequestInterface } from './Action';
import Cell from './Cell';
import { PieceType } from './Piece';

export default class PromotionAction extends Action {
  piece: PieceType;

  constructor(from: Cell, to: Cell, piece: PieceType) {
    super(from, to);
    this.piece = piece;
  }

  static fromActionInterface(ai: ActionInterface): PromotionAction {
    const data = ai.Promotion;
    const from = new Cell(data.color === 'Black' ? 1 : 6, data.start_column);
    const directionOffsets = {
      Straight: 0,
      Left: -1,
      Right: 1,
    };
    const to = new Cell(data.color === 'Black' ? 0 : 7, data.start_column + directionOffsets[data.direction]);
    const pieces = {
      Knight: PieceType.Knight,
      Rook: PieceType.Rook,
      Bishop: PieceType.Bishop,
      Queen: PieceType.Queen,
    };
    return new PromotionAction(from, to, pieces[data.promote_piece]);
  }

  asArray(): number[] {
    const pieces = {
      [PieceType.Knight]: 1,
      [PieceType.Rook]: 2,
      [PieceType.Bishop]: 6,
      [PieceType.Queen]: 7,
    };

    // encode promotion data in destination cell
    const to = new Cell(
      (this.from.row + 4 + this.from.column - this.to.column) % 8,
      (this.from.column + pieces[this.piece as keyof typeof pieces]) % 8,
    );

    return [this.from.asUint(), to.asUint()];
  }

  static fromArray(a: number[]): PromotionAction | undefined {
    const from = Cell.fromUint(a[0]);
    const to = Cell.fromUint(a[1]);

    // promotion direction and piece are encoded with cell offset
    const rowOffset = (8 + to.row - from.row) % 8;
    const colOffset = (8 + to.column - from.column) % 8;

    // decoding bounds
    if (
      (from.row !== 1 && from.row !== 6)
      || colOffset <= 0
      || (colOffset > 2 && colOffset < 6)
      || rowOffset < 3 || rowOffset > 5
    ) {
      return undefined;
    }

    const colOffsetToPiece = {
      1: PieceType.Knight,
      2: PieceType.Rook,
      6: PieceType.Bishop,
      7: PieceType.Queen,
    };

    const toRow = from.row === 6 ? 7 : 0;
    const toCol = from.column - rowOffset + 4;

    const promTo = new Cell(toRow, toCol);
    const piece = colOffsetToPiece[colOffset as keyof typeof colOffsetToPiece];

    return new PromotionAction(from, promTo, piece);
  }

  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to, piece: this.piece };
  }

  toAlgebraicNotation(): string {
    if (this.from.column === this.to.column) {
      return `${this.to.toCellName()}${this.piece.toUpperCase()}`;
    }
    return `${this.from.toCellName()[0]}×${this.to.toCellName()}${this.piece.toUpperCase()}`;
  }
}
