mod cfr;
mod game;

use cfr::history::*;
use cfr::trainer::train;
use game::kuhn::*;

fn main() {
    println!("Hello, world!");

    let root = KuhnPokerHistory::new();
    println!("{:?}", root.get_info_set());

    train(root, 1);
}
