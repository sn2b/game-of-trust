use crate::game::Action;
use super::strategy::Strategy;

pub struct Cheater;

impl Cheater { pub fn new() -> Self { Cheater } }

impl Strategy for Cheater {
    fn name(&self) -> &'static str { "Cheater" }
    fn next_move(&mut self, _my_history: &[Action], _their_history: &[Action]) -> Action {
        Action::Cheat
    }
    fn reset(&mut self) {}
}

pub fn create() -> Box<dyn Strategy> { Box::new(Cheater::new()) }
