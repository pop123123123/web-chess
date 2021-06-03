export enum PieceType {
  Pawn = 'p',
  Knight = 'n',
  Bishop = 'b',
  Rook = 'r',
  Queen = 'q',
  King = 'k',
}

export enum PieceColor {
  Light = 'l',
  Dark = 'd',
}

export default interface Piece {
  id: string
  type: PieceType
  color: PieceColor
  row: number
  column: number
  moving: boolean
  dead: boolean
}
