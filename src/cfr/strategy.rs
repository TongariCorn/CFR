use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::hash::Hash;

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

    updated: bool,
}

impl StrategyNode {
    fn new(n_act: usize) -> StrategyNode {
        return StrategyNode {
            dist: vec![1.0 / (n_act as f32); n_act],
            regret: vec![0.0; n_act],
            updated: false,
        };
    }

    pub fn get_prob(&self, act: usize) -> f32 {
        return if let Some(prob) = self.dist.get(act) {
            *prob
        } else {
            0.0
        };
    }
}

pub struct Strategy<Info: Eq + Hash + Copy> {
    nodes: HashMap<Info, RefCell<StrategyNode>>,
}

impl<Info: Eq + Hash + Copy> Strategy<Info> {
    pub fn new() -> Strategy<Info> {
        Strategy {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, info: Info, n_act: usize) {
        self.nodes.insert(info, RefCell::new(StrategyNode::new(n_act)));
    }

    pub fn get_node(&self, info: Info, n_act: usize) -> RefMut<StrategyNode> {
        return self.nodes.get(&info).unwrap().borrow_mut();
    }

    // pub fn get_dist(&mut self, info: Info, n_act: usize) -> &Vec<f32> {
    //     if !self.nodes.contains_key(&info) {
    //         self.nodes.insert(info, RefCell::new(StrategyNode::new(n_act)));
    //     }

    //     return &self.nodes.get(&info).unwrap().borrow().dist;
    // }

    // This function should be excuted only when all regrets at current round
    // are accumulated.
    pub fn regret_matching(&mut self) {}
}
