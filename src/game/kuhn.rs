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
}
