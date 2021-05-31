import Action from './Action';

export type GameId = number;

export default interface Game {
    history: Action[],
}
