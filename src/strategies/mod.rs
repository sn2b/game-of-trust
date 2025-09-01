pub mod strategy;
pub mod random;
pub mod copycat;
pub mod grudger;
pub mod cheater;
pub mod cooperator;
pub mod copykitten;
pub mod detective;

pub use strategy::Strategy;

pub struct StrategyFactory {
    pub name: &'static str,
    pub make: fn() -> Box<dyn Strategy>,
}

pub fn get_factories() -> Vec<StrategyFactory> {
    vec![
        StrategyFactory { name: "Random", make: random::create },
        StrategyFactory { name: "Copycat", make: copycat::create },
        StrategyFactory { name: "Grudger", make: grudger::create },
        StrategyFactory { name: "Cheater", make: cheater::create },
        StrategyFactory { name: "Cooperator", make: cooperator::create },
        StrategyFactory { name: "Copykitten", make: copykitten::create },
        StrategyFactory { name: "Detective", make: detective::create },
    ]
}
