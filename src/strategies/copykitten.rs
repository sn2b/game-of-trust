use crate::game::Action;
use super::strategy::Strategy;

pub struct Copykitten;

impl Copykitten { pub fn new() -> Self { Copykitten } }

impl Strategy for Copykitten {
    fn name(&self) -> &'static str { "Copykitten" }
    fn next_move(&mut self, _my_history: &[Action], their_history: &[Action]) -> Action {
        let len = their_history.len();
        if len >= 2 && their_history[len - 1] == Action::Cheat && their_history[len - 2] == Action::Cheat {
            Action::Cheat
        } else {
            Action::Cooperate
        }
    }
    fn reset(&mut self) {}
}

pub fn create() -> Box<dyn Strategy> { Box::new(Copykitten::new()) }
