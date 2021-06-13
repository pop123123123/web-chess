import { Action } from './Action';
import { PieceColor } from './Piece';

export default interface Promotion {
  action: Action
  color: PieceColor
}
