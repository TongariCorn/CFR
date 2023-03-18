use std::hash::Hash;
use std::collections::HashMap;

use crate::cfr::history::History;
use crate::cfr::strategy::Strategy;

fn cfr<H,I>(h: &H, target_player: usize) -> f32
where H: History<Info = I>,
      I: Eq + Hash + Copy {

    if h.is_terminal() {
        return h.get_utility(target_player)
    }

    let player = h.get_current_player();

    if player == 0 {    // chance player
        let mut cf_value = 0.0;
        for act in 0..h.get_action_num() {
            let next_h = h.take_action(act);
            let chance_probability = h.get_chance_probability(act);
            cf_value += chance_probability * cfr(&next_h, target_player);
        }
        return cf_value
    }

    return 0.0
}

pub fn train<H,I>(root: H, round: u8) 
where H: History<Info = I>,
      I: Eq + Hash + Copy {
    let mut strategy: HashMap<I, Vec<f32>> = HashMap::new();

    strategy.insert(root.get_info_set(), vec![0.0, 1.0]);

    let mut str = Strategy::<I>::new();
    str.get_dist(root.get_info_set(), 3);
    
    for i in 0..round {
        for target_player in 1..root.get_player_num() {
            cfr(&root, target_player);
        }
    }
}
