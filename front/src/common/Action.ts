import Cell from './Cell';

export default interface Action {
  from: Cell,
  to: Cell,
}
