use std::io;

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

    let mut h = Box::new(KuhnPokerHistory::new());

    println!("Which player would you like to play?: First(1)/Second(2)");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans);
    let p: usize = ans.trim().parse().ok().unwrap();
    let player = if p == 1 { 1 } else { 2 };
    let opponent = if player == 1 { 2 } else { 1 };

    let mut results = [0.0; 2];
    loop {
        let current_player = h.get_current_player();
        if current_player == opponent {
            println!("# Opponent's turn:")
        }

        let act = match current_player {
            0 => {
                // dealer
                h.sample_chance_probability()
            }
            _ => {
                if current_player == player {
                    println!("# Your turn, choose an action: {}", h.get_playable_actions_text());

                    let mut ans = String::new();
                    io::stdin().read_line(&mut ans);
                    let act = ans.trim().parse().ok().unwrap();

                    act
                } else {
                    strategy.sample_avg_strategy(h.get_info_set())
                }
            }
        };

        h = Box::new(h.take_action(act));

        h.print_playable_text(player);

        if h.is_terminal() {
            println!("The game is over.");

            results[player - 1] += h.get_utility(player);
            results[opponent - 1] += h.get_utility(opponent);
            println!(
                "Result --- You:{}({}) / Opponent:{}({})\n",
                results[player - 1],
                h.get_utility(player),
                results[opponent - 1],
                h.get_utility(opponent)
            );
            h = Box::new(KuhnPokerHistory::new());
        }
    }
}
