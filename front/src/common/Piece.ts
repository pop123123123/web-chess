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

export function getPieceImage(name: string): string {
  const images = require.context('../assets/pieces/', false, /\.svg$/);
  if (name.match(/[bknpqr][dl]/)) {
    return images(`./${name}.svg`);
  }
  return '';
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
