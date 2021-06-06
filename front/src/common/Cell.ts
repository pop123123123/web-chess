const baseCode = 'a'.charCodeAt(0);
export default class Cell {
  row: number;

  column: number;

  constructor(row: number, column: number) {
    this.row = row;
    this.column = column;
  }

  toCellName(): string {
    const columnName = String.fromCharCode(this.column + baseCode);
    return `${columnName}${this.row + 1}`;
  }

  asUint(): number {
    return this.row * 8 + this.column;
  }

  static fromUint(n: number): Cell {
    const row = Math.floor(n / 8);
    const column = n % 8;
    return new Cell(row, column);
  }
}
