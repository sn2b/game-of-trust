use crate::game::Action;
use super::strategy::Strategy;

pub struct Copycat;

impl Copycat { pub fn new() -> Self { Copycat } }

impl Strategy for Copycat {
    fn name(&self) -> &'static str { "Copycat" }
    fn next_move(&mut self, _my_history: &[Action], their_history: &[Action]) -> Action {
        their_history.last().cloned().unwrap_or(Action::Cooperate)
    }
    fn reset(&mut self) {}
}

pub fn create() -> Box<dyn Strategy> { Box::new(Copycat::new()) }
