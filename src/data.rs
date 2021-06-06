use crate::rand::RngCore;
use chashmap::CHashMap;
use web_chess::board::{Action, Game};

pub type GameId = u32;
pub type GameData = CHashMap<GameId, Game>;

/// Create a new game and store it, returning its ID
pub fn create(data: &GameData, history: Vec<Action>) -> GameId {
    let mut rng = rand::thread_rng();
    // generate a random number not present in hashmap
    let num: GameId = loop {
        let x = rng.next_u32();
        if !data.contains_key(&x) {
            break x;
        }
    };
    data.insert(num, Game::from(history));
    num
}
