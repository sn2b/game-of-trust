use crate::game::Action;
use super::strategy::Strategy;

pub struct Grudger {
    grudge: bool,
}

impl Grudger {
    pub fn new() -> Self { Grudger { grudge: false } }
}

impl Strategy for Grudger {
    fn name(&self) -> &'static str { "Grudger" }
    fn next_move(&mut self, _my_history: &[Action], their_history: &[Action]) -> Action {
        if self.grudge { return Action::Cheat; }
        if their_history.iter().any(|&a| a == Action::Cheat) {
            self.grudge = true;
            Action::Cheat
        } else {
            Action::Cooperate
        }
    }
    fn reset(&mut self) { self.grudge = false; }
}

pub fn create() -> Box<dyn Strategy> { Box::new(Grudger::new()) }
