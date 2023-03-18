pub trait History {
    type Info;

    fn get_info_set(&self) -> Self::Info;

    fn is_terminal(&self) -> bool;
    fn get_utility(&self, player: usize) -> f32;

    fn get_action_num(&self) -> usize;
    fn take_action(&self, act: usize) -> Self;

    fn get_player_num(&self) -> usize;  // the number of player + chance player (dealer)
    fn get_current_player(&self) -> usize;
    fn get_chance_probability(&self, act: usize) -> f32;
}