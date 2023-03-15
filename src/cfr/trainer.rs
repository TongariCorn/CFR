use std::hash::Hash;
use std::collections::HashMap;

use crate::cfr::history::History;
use crate::cfr::strategy::Strategy;

pub fn train<H,I>(root: H, round: u8) 
where H: History<Info = I>,
      I: Eq + Hash + Copy {
    let mut strategy: HashMap<I, Vec<f32>> = HashMap::new();

    strategy.insert(root.get_info_set(), vec![0.0, 1.0]);

    let mut str = Strategy::<I>::new();
    str.get_dist(root.get_info_set(), 3);
}
