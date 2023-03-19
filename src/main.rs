use std::io;

mod cfr;
mod game;

use cfr::history::*;
use cfr::trainer::train;
use game::kuhn::*;

fn main() {
    let root = KuhnPokerHistory::new();

    let mut strategy = KuhnPokerHistory::new_strategy();
    train(root, &mut strategy, 500);

    let mut h = Box::new(KuhnPokerHistory::new());

    let mut player = 1;
    let mut opponent = 2;
    let mut game_iter = 1;

    println!("===Game{}===   You play first.", game_iter);

    let mut results = [0.0; 2];
    loop {
        let current_player = h.get_current_player();

        let act = match current_player {
            0 => {
                // dealer
                h.sample_chance_probability()
            }
            _ => {
                if current_player == player {
                    println!(
                        "# Your turn, choose an action: {}",
                        h.get_playable_actions_text()
                    );

                    let mut ans = String::new();
                    io::stdin().read_line(&mut ans);
                    let act = ans.trim().parse().ok().unwrap();

                    act
                } else {
                    let act = strategy.sample_avg_strategy(h.get_info_set());
                    println!("# Opponent's turn: {}", h.get_action_as_text(act));

                    act
                }
            }
        };

        h = Box::new(h.take_action(act));

        println!("{}", h.get_info_text(player));

        if h.is_terminal() {
            println!("The game is over.");

            results[0] += h.get_utility(player);
            results[1] += h.get_utility(opponent);
            println!(
                "Result --- You:{}({}) / Opponent:{}({})\n",
                results[0],
                h.get_utility(player),
                results[1],
                h.get_utility(opponent)
            );

            // New game
            h = Box::new(KuhnPokerHistory::new());
            game_iter += 1;
            let temp = player;
            player = opponent;
            opponent = temp;
            println!(
                "===Game{}===   {} first.",
                game_iter,
                if player == 1 {
                    "You play"
                } else {
                    "The opponent plays"
                }
            );
        }
    }
}
