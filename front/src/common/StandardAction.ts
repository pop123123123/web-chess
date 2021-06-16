import { Action, ActionInterface } from './Action';

export default class StandardAction extends Action {
  serialize(): ActionInterface {
    return { Standard: { from: this.from, to: this.to } };
  }
}
