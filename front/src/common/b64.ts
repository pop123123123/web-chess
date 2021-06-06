import { Action } from './Action';

const BASE64_ENCODE = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=';
const BASE64_DECODE = Object.fromEntries(Array.from(BASE64_ENCODE).map((v, k: number) => [v, k]));

function historyFromBase64(b: string): Action[] {
  const history: number[] = Array.from(b).map((c: string) => BASE64_DECODE[c]);
  const actions: Action[] = [];
  for (let i = 0; i < b.length; i += 2) {
    actions.push(Action.fromArray(history.slice(i, i + 2)));
  }
  return actions;
}

export { BASE64_ENCODE, BASE64_DECODE, historyFromBase64 };
