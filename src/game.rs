use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Cooperate,
    Cheat,
}

pub fn payoff(a: Action, b: Action) -> (i32, i32) {
    match (a, b) {
        (Action::Cooperate, Action::Cooperate) => (3, 3),
        (Action::Cooperate, Action::Cheat) => (0, 5),
        (Action::Cheat, Action::Cooperate) => (5, 0),
        (Action::Cheat, Action::Cheat) => (1, 1),
    }
}

pub type History = Vec<Action>;
