// Ref: https://www.reddit.com/r/dailyprogrammer/comments/n94io8/20210510_challenge_389_easy_the_monty_hall_problem/

use rand::{Rng, rngs::ThreadRng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
enum Player {
    Alice,
    Bob,
    Carol,
    Dave,
    Erin,
    Frank,
}

impl Player {
    fn monty(&self, rng: &mut ThreadRng) -> bool {
        // Step 1
        let prize = rng.random_range(0..=2);

        // Step 2
        let initial_guess = match self {
            Player::Alice => 0,
            Player::Bob => 0,
            Player::Carol => rng.random_range(0..=2),
            Player::Dave => rng.random_range(0..=2),
            Player::Erin => rng.random_range(0..=2),
            Player::Frank => 0,
        };

        // Step 3
        let revealed_door = if prize != initial_guess {
            3 - prize - initial_guess
        } else {
            (prize + rng.random_range(1..=2)) % 3
        };

        // Step 3.5
        let switched = 3 - revealed_door - initial_guess;

        // Step 4
        let final_guess = match self {
            Player::Alice => 0,
            Player::Bob => switched,
            Player::Carol => if rng.random_bool(0.5) { initial_guess } else { switched }
            Player::Dave => initial_guess,
            Player::Erin => switched,
            Player::Frank => if revealed_door == 1 { 0 } else { 1 }
        };

        // Step 5
        final_guess == prize
    }

    fn simulate(&self, trials: u32, rng: &mut ThreadRng) -> u32 {
        let mut wins = 0u32;
        for _ in 0..trials {
            wins += self.monty(rng) as u32;
        }
        wins
    }
}

struct Gina {
    mode: Player,
}

impl Gina {
    fn new() -> Self {
        Self {
            mode: Player::Alice,
        }
    }

    fn simulate(&mut self, trials: u32, rng: &mut ThreadRng) -> u32 {
        debug_assert!(self.mode == Player::Alice || self.mode == Player::Bob);
        let mut wins = 0;
        for _ in 0..trials {
            let result = self.mode.monty(rng) as u32;
            wins += result;

            if result == 0 {
                self.mode = if self.mode == Player::Alice {
                    Player::Bob
                } else {
                    Player::Alice
                };
            }
        }
        wins
    }
}

fn main() {
    let mut rng = rand::rng();
    let trials = 100_000;

    for player in Player::iter() {
        let wins = player.simulate(trials, &mut rng);
        println!("{:?}: {:.3}", player, wins as f64 / trials as f64);
    }

    let mut gina = Gina::new();
    let gina_wins = gina.simulate(trials, &mut rng);
    println!("Gina: {:.3}", gina_wins as f64 / trials as f64);
}
