use crate::game::Action;

pub trait Strategy {
    /// Static name for the strategy
    fn name(&self) -> &'static str;

    /// Decide the next move given the complete history up to (but not including) this round.
    fn next_move(&mut self, my_history: &[Action], their_history: &[Action]) -> Action;

    /// Reset internal state before a new match (optional).
    fn reset(&mut self) {}
}
