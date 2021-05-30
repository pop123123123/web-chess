use crate::rand::RngCore;
use chashmap::CHashMap;
use web_chess::board::Game;

pub type Id = u32;
pub type GameData = CHashMap<u32, Game>;

pub fn create(data: &GameData) -> u32 {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.next_u32();
    // while exists
    data.insert(num, Game::new());
    num
}
