use itertools::Itertools;

use crate::cfr::{history::History, strategy::Strategy};

// 0...there is no card or the card is unobservable
type Card = u8;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(super) enum Opt {
    CHECK,
    BET,
    CALL,
    FOLD,
    NULL,
}

type PlayerState = (Card, Opt);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct KuhnPokerHistory {
    is_terminal: bool,
    current_player: usize,
    turn: usize,

    board: Card,
    player_states: [PlayerState; 2],
}

impl KuhnPokerHistory {
    pub fn new() -> Self {
        KuhnPokerHistory {
            is_terminal: false,
            current_player: 0,
            turn: 0,
            board: 0,
            player_states: [(0, Opt::NULL), (0, Opt::NULL)],
        }
    }
}

impl History for KuhnPokerHistory {
    type Info = KuhnPokerHistory;

    fn get_info_set(&self) -> KuhnPokerHistory {
        let mut info = *self;
        if info.current_player == 1 {
            info.player_states[1].0 = 0;
        } else {
            info.player_states[0].0 = 0;
        }
        return info
    }

    fn new_strategy() -> Strategy<KuhnPokerHistory> {
        let mut str = Strategy::new();
        let root = KuhnPokerHistory::new();

        fn dfs(h: KuhnPokerHistory, str: &mut Strategy<KuhnPokerHistory>) {
            if h.is_terminal() { return }
            if h.get_current_player() != 0 { str.add_node(h.get_info_set(), h.get_action_num()) };
            for act in 0..h.get_action_num() {
                let next_h = h.take_action(act);
                dfs(next_h, str);
            }
        }
        dfs(root, &mut str);

        return str
    }

    fn is_terminal(&self) -> bool {
        return self.is_terminal;
    }

    fn get_utility(&self, player: usize) -> f32 {
        if !self.is_terminal || player > 2 || player == 0 {
            return 0.0;
        } else {
            let ret = match (self.player_states[0].1, self.player_states[1].1) {
                (Opt::CALL, _) | (_, Opt::CALL) => 2.0,
                _ => 1.0,
            };
            match (self.player_states[0].1, self.player_states[1].1) {
                (Opt::FOLD, _) => {
                    // player 2 wins
                    if player == 1 {
                        return -1.0;
                    } else {
                        return 1.0;
                    }
                }
                (_, Opt::FOLD) => {
                    // player 1 wins
                    if player == 1 {
                        return 1.0;
                    } else {
                        return -1.0;
                    }
                }
                _ => {
                    // showdown
                    if player == 1 {
                        if self.player_states[0].0 > self.player_states[1].0 {
                            return ret;
                        } else {
                            return -ret;
                        }
                    } else {
                        if self.player_states[0].0 > self.player_states[1].0 {
                            return -ret;
                        } else {
                            return ret;
                        }
                    }
                }
            }
        }
    }

    fn get_action_num(&self) -> usize {
        return match self.turn {
            0 => 6, // the number of all permutations of J,Q,K
            1 => 2, // check or bet
            2 => 2, // (check or bet) or (fold or call)
            3 if !self.is_terminal => 2,
            _ => 0,
        };
    }

    fn take_action(&self, act: usize) -> Self {
        if self.is_terminal() {
            return *self;
        }

        let mut next_h = *self;

        match self.turn {
            0 => {
                // deal cards
                next_h.turn = 1;
                let cards = &(11..=13).permutations(3).collect_vec()[act];
                next_h.board = cards[0];
                next_h.player_states[0].0 = cards[1];
                next_h.player_states[1].0 = cards[2];
                next_h.current_player = 1;
            }
            1 => {
                // player 1 turn
                next_h.turn = 2;
                next_h.current_player = 2;
                if act == 0 {
                    // check
                    next_h.player_states[0].1 = Opt::CHECK;
                } else {
                    // bet
                    next_h.player_states[0].1 = Opt::BET;
                }
            }
            2 => {
                // player 2 turn
                next_h.turn = 3;
                match self.player_states[0].1 {
                    Opt::CHECK => {
                        if act == 0 {
                            // check
                            // showdown
                            next_h.player_states[1].1 = Opt::CHECK;
                            next_h.is_terminal = true;
                        } else {
                            next_h.player_states[1].1 = Opt::BET;
                            next_h.current_player = 1;
                        }
                    }
                    _ => {
                        // bet
                        if act == 0 {
                            // fold
                            next_h.player_states[1].1 = Opt::FOLD;
                            next_h.is_terminal = true;
                        } else {
                            // call
                            // showdown
                            next_h.player_states[1].1 = Opt::CALL;
                            next_h.is_terminal = true;
                        }
                    }
                }
            }
            3 => {
                // player 1 turn
                next_h.turn = 4;
                if act == 0 {
                    // fold
                    next_h.player_states[0].1 = Opt::FOLD;
                    next_h.is_terminal = true;
                } else {
                    // call
                    // showdown
                    next_h.player_states[0].1 = Opt::CALL;
                    next_h.is_terminal = true;
                }
            }
            _ => (),
        }
        return next_h;
    }

    fn get_player_num(&self) -> usize {
        return 3;
    }

    fn get_current_player(&self) -> usize {
        return self.current_player;
    }

    fn get_chance_probability(&self, _act: usize) -> f32 {
        return match self.turn {
            0 => 1.0 / 6.0, // the number of all permutations of J,Q,K
            _ => 0.0,
        };
    }
}
