use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp;

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

    unnormalized_avg_strategy: Vec<f32>,
    avg_strategy: Vec<f32>,

    updated: bool,
}

impl StrategyNode {
    fn new(n_act: usize) -> StrategyNode {
        return StrategyNode {
            dist: vec![1.0 / (n_act as f32); n_act],
            regret: vec![0.0; n_act],
            unnormalized_avg_strategy: vec![0.0; n_act],
            avg_strategy: vec![0.0; n_act],
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

    pub fn update_regret(&mut self, act: usize, r: f32, pi_i: f32, prob: f32) {
        self.regret[act] += r;
        self.unnormalized_avg_strategy[act] += pi_i * prob;
        self.updated = true;
    }
    
    pub fn add_imm_cfr(&mut self, act: usize, r: f32) {
        self.regret[act] += r;
    }
    
    pub fn accum_avg_strategy(&mut self, act: usize, pi_i: f32, prob: f32) {
        self.unnormalized_avg_strategy[act] += pi_i * prob;
        self.updated = true;
    }
    
    fn regret_matching(&mut self) {
        if !self.updated { return }

        let mut normalizing_sum = 0.0;
        for act in 0..self.regret.len() {
            self.dist[act] = if self.regret[act] > 0.0 { self.regret[act] } else { 0.0 };
            normalizing_sum += self.dist[act];
        }

        // normalize self.dist
        for act in 0..self.dist.len() {
            if normalizing_sum > 0.0 {
                self.dist[act] /= normalizing_sum;
            } else {
                self.dist[act] = 1.0 / (self.dist.len() as f32);
            }
        }

        self.updated = false;
    }

    fn calc_average_strategy(&mut self) {
        let normalizing_sum: f32 = self.unnormalized_avg_strategy.iter().sum();
        for act in 0..self.unnormalized_avg_strategy.len() {
            self.avg_strategy[act] = self.unnormalized_avg_strategy[act] / normalizing_sum;
        }
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
        self.nodes
            .insert(info, RefCell::new(StrategyNode::new(n_act)));
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
    pub fn regret_matching(&mut self) {
        for (_, v) in self.nodes.iter_mut() {
            v.borrow_mut().regret_matching();
        }
    }

    pub fn calc_average_strategy(&mut self) {
        for (_, v) in self.nodes.iter_mut() {
            v.borrow_mut().calc_average_strategy();
        }
    }
}
