use crate::game::Action;
use super::strategy::Strategy;

pub struct Detective {
    round: usize,
    opponent_ever_cheated: bool,
}

impl Detective {
    pub fn new() -> Self { Detective { round: 0, opponent_ever_cheated: false } }
}

impl Strategy for Detective {
    fn name(&self) -> &'static str { "Detective" }
    fn next_move(&mut self, _my_history: &[Action], their_history: &[Action]) -> Action {
        // initial moves: C, C, D, C
        let choice = match self.round {
            0 => Action::Cooperate,
            1 => Action::Cooperate,
            2 => Action::Cheat,
            3 => Action::Cooperate,
            _ => {
                if self.opponent_ever_cheated {
                    // behave like copycat
                    their_history.last().cloned().unwrap_or(Action::Cooperate)
                } else {
                    Action::Cheat
                }
            }
        };
        // update opponent_ever_cheated based on their_history at this point
        if their_history.iter().any(|&a| a == Action::Cheat) {
            self.opponent_ever_cheated = true;
        }
        self.round += 1;
        choice
    }
    fn reset(&mut self) { self.round = 0; self.opponent_ever_cheated = false; }
}

pub fn create() -> Box<dyn Strategy> { Box::new(Detective::new()) }
