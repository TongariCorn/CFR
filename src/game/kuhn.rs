use crate::cfr::history::History;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct KuhnPokerHistory {
}

impl KuhnPokerHistory {
    pub fn new() -> Self {
        KuhnPokerHistory {}
    }
}

impl History for KuhnPokerHistory {
    type Info = KuhnPokerHistory;

    fn get_info_set(&self) -> KuhnPokerHistory {
        return KuhnPokerHistory {  }
    }

    fn is_terminal(&self) -> bool {
        return true
    }

    fn get_utility(&self, player: usize) -> f32 {
        return 0.0
    }

    fn get_action_num(&self) -> usize {
        return 0
    }

    fn take_action(&self, act: usize) -> Self {
        return KuhnPokerHistory {  }
    }

    fn get_player_num(&self) -> usize {
        return 3
    }

    fn get_current_player(&self) -> usize {
        return 0
    }

    fn get_chance_probability(&self, act: usize) -> f32 {
        return 0.0
    }
}
