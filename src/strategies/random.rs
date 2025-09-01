use rand::Rng;
use crate::game::Action;
use super::strategy::Strategy;

pub struct Random {
    rng: rand::rngs::ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Self { rng: rand::thread_rng() }
    }
}

impl Strategy for Random {
    fn name(&self) -> &'static str { "Random" }

    fn next_move(&mut self, _my_history: &[Action], _their_history: &[Action]) -> Action {
        if self.rng.gen_bool(0.5) { Action::Cooperate } else { Action::Cheat }
    }

    fn reset(&mut self) {}
}

pub fn create() -> Box<dyn Strategy> { Box::new(Random::new()) }
