use crate::rand::RngCore;
use chashmap::CHashMap;
use web_chess::board::Game;

pub type Id = u32;
pub type GameData = CHashMap<u32, Game>;

pub fn create(data: &GameData) -> u32 {
    let mut rng = rand::thread_rng();
    // generate a random number not present in hashmap
    let num: u32 = loop {
        let x = rng.next_u32();
        if !data.contains_key(&x) {
            break x;
        }
    };
    data.insert(num, Game::new());
    num
}
