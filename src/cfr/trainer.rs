use std::hash::Hash;
use std::collections::HashMap;

use crate::cfr::history::History;
use crate::cfr::strategy::Strategy;

// # Output
// averaged utility at current node - \sum_{h \sqsubseteq z} \pi^\sigma(h,z)u_i(z)
fn cfr<H,I>(h: &H, str: &Strategy<I>, target_player: usize) -> f32
where H: History<Info = I>,
      I: Eq + Hash + Copy {

    if h.is_terminal() {
        return h.get_utility(target_player)
    }

    let player = h.get_current_player();

    if player == 0 {    // chance player
        let mut avg_utility = 0.0;
        for act in 0..h.get_action_num() {
            let next_h = h.take_action(act);
            let chance_prob = h.get_chance_probability(act);
            avg_utility += chance_prob * cfr(&next_h, str, target_player);
        }
        return avg_utility
    }

    let info = h.get_info_set();
    let node = str.get_node(info, h.get_action_num());

    let mut avg_utilities = vec![0.0; h.get_action_num()];
    let mut avg_utility = 0.0;
    for act in 0..h.get_action_num() {
        let next_h = h.take_action(act);
        if player == target_player {
            avg_utilities[act] = cfr(&next_h, str, target_player);
        } else {
            avg_utilities[act] = cfr(&next_h, str, target_player);
        }
        avg_utility += node.get_prob(act) * avg_utilities[act];
    }

    return avg_utility
}

pub fn train<H,I>(root: H, round: u8) 
where H: History<Info = I>,
      I: Eq + Hash + Copy {
    let mut strategy: HashMap<I, Vec<f32>> = HashMap::new();

    strategy.insert(root.get_info_set(), vec![0.0, 1.0]);

    let mut str = H::new_strategy();
    
    for i in 0..round {
        for target_player in 1..root.get_player_num() {
            cfr(&root, &mut str, target_player);
        }
    }
}
