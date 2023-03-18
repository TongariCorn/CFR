extern crate rand;

use rand::Rng;

use crate::cfr::strategy::Strategy;
use std::hash::Hash;

pub trait History {
    type Info: Eq + Hash + Copy;

    fn get_info_set(&self) -> Self::Info;
    fn new_strategy() -> Strategy<Self::Info>;

    fn is_terminal(&self) -> bool;
    fn get_utility(&self, player: usize) -> f32;

    fn get_action_num(&self) -> usize;
    fn take_action(&self, act: usize) -> Self;

    // the number of player + chance player (dealer)
    fn get_player_num(&self) -> usize;
    fn get_current_player(&self) -> usize;
    fn get_chance_probability(&self, act: usize) -> f32;

    fn sample_chance_probability(&self) -> usize {
        let mut rng = rand::thread_rng();
        let sample = rng.gen::<f64>();

        let mut accum_p = 0.0;
        for act in 0..self.get_action_num() {
            accum_p += self.get_chance_probability(act);
            if sample <= (accum_p as f64) {
                return act;
            }
        }
        return self.get_action_num() - 1;
    }
}
