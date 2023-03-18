use std::hash::Hash;

use crate::cfr::history::History;
use crate::cfr::strategy::Strategy;

// # Input
// * pi_i  - \pi^\sigma_i(h): the contribution from the target player (i.e., player i)
// * pi_mi - \pi^\sigma_{-i}(h): the product of all players' contribution except that of the target player (i.e., player i)
// # Output
// averaged utility at current node - \sum_{h \sqsubseteq z} \pi^\sigma(h,z)u_i(z)
fn cfr<H, I>(h: &H, strategy: &Strategy<I>, target_player: usize, pi_i: f32, pi_mi: f32) -> f32
where
    H: History<Info = I>,
    I: Eq + Hash + Copy,
{
    if h.is_terminal() {
        return h.get_utility(target_player);
    }

    let player = h.get_current_player();

    if player == 0 {
        // chance player
        let mut avg_utility = 0.0;
        for act in 0..h.get_action_num() {
            let next_h = h.take_action(act);
            let chance_prob = h.get_chance_probability(act);
            avg_utility +=
                chance_prob * cfr(&next_h, strategy, target_player, pi_i, pi_mi * chance_prob);
        }
        return avg_utility;
    }

    // all players except chance player
    let info = h.get_info_set();
    let mut node = strategy.get_node(info, h.get_action_num());

    let mut avg_utilities = vec![0.0; h.get_action_num()];
    let mut avg_utility = 0.0;
    for act in 0..h.get_action_num() {
        let next_h = h.take_action(act);
        if player == target_player {
            avg_utilities[act] = cfr(
                &next_h,
                strategy,
                target_player,
                pi_i * node.get_prob(act),
                pi_mi,
            );
        } else {
            avg_utilities[act] = cfr(
                &next_h,
                strategy,
                target_player,
                pi_i,
                pi_mi * node.get_prob(act),
            );
        }
        avg_utility += node.get_prob(act) * avg_utilities[act];
    }

    if player == target_player {
        // calculate immediate counterfactual regret
        for act in 0..h.get_action_num() {
            // immediate counterfactual regret for current history
            let regret = pi_mi * (avg_utilities[act] - avg_utility);
            let prob = node.get_prob(act);
            node.update_regret(act, regret, pi_i, prob);
        }
    }

    return avg_utility;
}

pub fn train<H, I>(root: H, strategy: &mut Strategy<I>, round: u32)
where
    H: History<Info = I>,
    I: Eq + Hash + Copy,
{

    for i in 0..round {
        for target_player in 1..root.get_player_num() {
            cfr(&root, strategy, target_player, 1.0, 1.0);

            // update strategy with regret matching
            strategy.regret_matching();
        }
    }

    strategy.calc_average_strategy();
}
