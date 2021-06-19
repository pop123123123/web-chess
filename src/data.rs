use crate::board::{ActionRequest, Game};
use chashmap::CHashMap;
use rand::RngCore;

pub type GameId = u32;
pub type GameData = CHashMap<GameId, Game>;

/// Create a new game and store it, returning its ID
pub fn create(data: &GameData, history: Vec<ActionRequest>) -> GameId {
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
