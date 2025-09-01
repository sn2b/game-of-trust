use crate::game::Action;
use super::strategy::Strategy;

pub struct Cooperator;

impl Cooperator { pub fn new() -> Self { Cooperator } }

impl Strategy for Cooperator {
    fn name(&self) -> &'static str { "Cooperator" }
    fn next_move(&mut self, _my_history: &[Action], _their_history: &[Action]) -> Action {
        Action::Cooperate
    }
    fn reset(&mut self) {}
}

pub fn create() -> Box<dyn Strategy> { Box::new(Cooperator::new()) }
