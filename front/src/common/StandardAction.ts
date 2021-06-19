import { Action, ActionRequestInterface } from './Action';

export default class StandardAction extends Action {
  serialize(): ActionRequestInterface {
    return { from: this.from, to: this.to };
  }
}
