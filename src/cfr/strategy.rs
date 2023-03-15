use std::hash::Hash;
use std::collections::HashMap;

pub struct Strategy<Info: Eq + Hash + Copy> {
    sigma: HashMap<Info, Vec<f32>>,
}

impl<Info: Eq + Hash + Copy> Strategy<Info> {
    pub fn new() -> Strategy<Info> {
        Strategy { sigma: HashMap::new() }
    }

    pub fn get_dist(&mut self, info: Info, n_act: usize) -> &Vec<f32> {
        if !self.sigma.contains_key(&info) {
            self.sigma.insert(info,
                              vec![1.0/(n_act as f32); n_act]);
        }

        return self.sigma.get(&info).unwrap()
    }
}
