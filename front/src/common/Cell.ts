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
    return `${columnName}${this.row}`;
  }
}
