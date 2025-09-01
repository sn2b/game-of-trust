# Game of Trust

## The game we are playing

The game we are playing is a game of trust, often referred to as the prisoner's dilemma. Two players play against each other, each with the option to cooperate or cheat.

| Choices                     | Player 1 ooperates | Player 1 cheats |
|-----------------------------|--------------------|-----------------------------|
| Player 2 cooperates          | +3 / +3            |  0 / +5                     |
| Player 2 cheats | +5 /  0            | +1 / +1                     |

Cooperating will lead to both players earning three points each. When one of the players cheats, they earn five points while the other earns nothing. If neither of the players cooperates, they will both earn one point.

## The strategies

| Name       | description                                                                                                                         | added by |
|------------|-------------------------------------------------------------------------------------------------------------------------------------|----------|
| Random     | Picks at random                                                                                                                     | @sn2b    |
| Copycat    | Copies the opponents last move                                                                                                      | @sn2b    |
| Grudger    | Cooperates, until the opponent cheats, then always cheats                                                                           | @sn2b    |
| Cheater    | Always cheats                                                                                                                       | @sn2b    |
| Cooperator | Always cooperates                                                                                                                   | @sn2b    |
| Copykitten | Cooperates, unless the opponent cheats twice, then cheats the next round                                                            | @sn2b    |
| Detective  | Starts with Cooperate, Cooperate, Cheat, Cooperate. Afterwards, if you ever retaliate plays like Copycat. Otherwise, always cheats. | @sn2b    |

## Project structure

This repository follows a small, conventional Rust layout with a few supporting folders for site generation and historic data. Key files and folders:

- `Cargo.toml` — the Rust package manifest.
- `src/` — Rust source code:
  - `src/main.rs` — binary entry (runs the tournament).
  - `src/lib.rs` — library exports.
  - `src/game.rs` — core game model (`Action`, `payoff`, history types).
  - `src/tournament.rs` — tournament runner: pairs strategies, runs randomized-length matches, normalizes scores, writes results.
  - `src/output.rs` — HTML generation for the leaderboard (keeps presentation isolated).
  - `src/strategies/` — a submodule containing each strategy in its own file plus the `Strategy` trait:
    - `strategy.rs` — the `Strategy` trait (`next_move(&mut self, my_history, their_history) -> Action`).
    - `<your_strategy>.rs` — individual strategies live here and expose a `create()` factory.
    - `mod.rs` — registers available strategies (factories used by the tournament).
- `data/` — historic tournament results (currently `history.json`).
- `static/` — generated static preview of the last tournament (good for local viewing).
- `docs/` — generated site content (published automatically to GitHub Pages by the workflow).
- `.github/workflows/` — GitHub Actions workflow that runs the weekly tournament and publishes the site.

This layout keeps the game logic, tournament orchestration and presentation separated so contributors can implement strategies without touching the rest of the system.

## The tournament

Each week, we will run a tournament through Actions in which all strategies will play each other. The leaderboard shows the results for the last week, while we will also keep track of the all time best score.

To prevent predictability in the last rounds, the tournament will have a random number of rounds, between 150 and 250. We will normalize the scores to reflect this.

## The leaderboard

The leaderboard will be on a Pages site here: https://sn2b.github.io/game-of-trust/

## How to contribute

Contributions welcome. Typical workflow to add a new strategy:

1. Fork the repository and create a branch for your strategy.
2. Add a file `src/strategies/my_strategy.rs` implementing the `Strategy` trait. Keep your strategy isolated in that file and expose a `create()` constructor used as a factory by `src/strategies/mod.rs`.

A minimal example strategy skeleton:

```rust
use crate::game::Action;
use super::strategy::Strategy;

pub struct MyStrategy;

impl MyStrategy {
    pub fn new() -> Self { MyStrategy }
}

impl Strategy for MyStrategy {
    fn name(&self) -> &'static str { "MyStrategy" }

    fn next_move(&mut self, _my_history: &[Action], _their_history: &[Action]) -> Action {
        // Implement your strategy here based on the history of moves
        Action::Cooperate
    }

    fn reset(&mut self) {}
}

pub fn create() -> Box<dyn Strategy> { Box::new(MyStrategy::new()) }
```

3. Register your strategy in `src/strategies/mod.rs` by adding a `StrategyFactory { name: "MyStrategy", make: my_strategy::create }` entry in the `get_factories()` array. This is how the tournament discovers available strategies.

4. Run the tournament locally to verify your strategy behaves as intended:

- `cargo run --release` — runs the tournament and writes `data/history.json`, `static/index.html` and `docs/`.
- Open `static/index.html` in a browser for a quick preview.

5. Open a PR with your new strategy, description and any rationale. Add yourself to the contributors column in the README or the strategies table if you want to be listed.

Guidelines and notes

- Strategies receive the complete history of past moves through the `my_history` and `their_history` parameters. Use this information to make informed decisions.
- If your strategy needs randomness, use `rand::thread_rng()` locally inside the strategy. The tournament runner itself randomizes match lengths and uses RNG in a tested way.
- Keep dependencies minimal. If your strategy requires an extra crate, add it to `Cargo.toml` and explain the reason in your PR.

Thanks for contributing — add your strategy with a PR and see how it does!
