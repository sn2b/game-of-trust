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

## The tournament

Each week, we will run a tournament through Actions in which all strategies will play each other. The leaderboard shows the results for the last week, while we will also keep track of the all time best score.

To prevent predictability in the last rounds, the tournament will have a random number of rounds, between 150 and 250. We will normalize the scores to reflect this.

## The leaderboard

The leaderboard will be on a Pages site here: https://sn2b.github.io/game-of-trust/

## How to contribute

Add your own strategy with a PR and see how it does.
