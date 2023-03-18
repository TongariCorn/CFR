use std::hash::Hash;
use std::collections::HashMap;

// StrategyNode encapsulates the following three quantities:
// 1. \sigma^t: strategy at rount t
// 2. \overline{\sigma}^t: overall strategy
// 3. R^t: immediate counterfactual regret accumulation
pub struct StrategyNode {
    // \sigma(a|I): distribution
    dist: Vec<f32>,

    // immediate counterfactual regret accumulation
    // r[I][a] = 1/T \sum_{t=1}^T (v(\sigma^t_{(I \to a)},I) - v(\sigma^t, I))
    regret: Vec<f32>,

    updated: bool
}

impl StrategyNode {
    fn new(n_act: usize) -> StrategyNode {
        return StrategyNode { dist: vec![1.0/(n_act as f32); n_act], regret: vec![0.0; n_act], updated: false }
    }
}

pub struct Strategy<Info: Eq + Hash + Copy> {
    node: HashMap<Info, StrategyNode>,
}

impl<Info: Eq + Hash + Copy> Strategy<Info> {
    pub fn new() -> Strategy<Info> {
        Strategy { node: HashMap::new() }
    }

    pub fn get_dist(&mut self, info: Info, n_act: usize) -> &Vec<f32> {
        if !self.node.contains_key(&info) {
            self.node.insert(
                info,
                StrategyNode::new(n_act));
        }

        return &self.node.get(&info).unwrap().dist
    }

    // This function should be excuted only when all regrets at current round 
    // are accumulated.
    pub fn regret_matching(&mut self) {
    }
}
