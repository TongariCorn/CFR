mod cfr;
mod game;

use cfr::history::*;
use cfr::trainer::train;
use game::kuhn::*;

fn main() {
    println!("Hello, world!");

    let root = KuhnPokerHistory::new();
    println!("{:?}", root.get_info_set());

    let mut strategy = KuhnPokerHistory::new_strategy();
    train(root, &mut strategy, 500);

    println!("yeah");
}
