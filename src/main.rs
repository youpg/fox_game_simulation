use rand::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::io::{self, Write};
use std::time::Instant;

// const NUM_OF_GAMES: u64 = 1_000_000_000;
const BATCH_SIZE: u64 = 1_000_000;

#[derive(Copy, Clone)]
struct Board([u8; 16]);

impl Board {
    #[inline(always)]
    fn new() -> Self {
        Board([0; 16])
    }

    #[inline(always)]
    fn fill_and_shuffle(&mut self, rng: &mut impl Rng) {
        let mut bag = [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3];
        bag.shuffle(rng);
        self.0.copy_from_slice(&bag);
    }

    #[inline(always)]
    fn check_for_word_fox(&self) -> bool {
        const DIRECTIONS: [(isize, isize); 8] = [
            (1, 0), (0, 1), (1, 1), (-1, 1),
            (-1, 0), (0, -1), (-1, -1), (1, -1)
        ];

        for start in 0..16 {
            let (x, y) = (start / 4, start % 4);
            for &(dx, dy) in &DIRECTIONS {
                let mut valid = true;
                let fox = [1, 2, 3];
                for i in 0..3 {
                    let nx = x as isize + i as isize * dx;
                    let ny = y as isize + i as isize * dy;
                    if nx < 0 || nx >= 4 || ny < 0 || ny >= 4 || self.0[nx as usize * 4 + ny as usize] != fox[i] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let mut num_of_games = String::new();
    print!("Enter the number of games to simulate: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num_of_games).unwrap();
    let num_of_games: u64 = num_of_games.trim().parse().unwrap_or(0);

    let fox_found = AtomicU64::new(0);

    let full_batches = num_of_games / BATCH_SIZE;
    let remainder = num_of_games % BATCH_SIZE;

    let start_time = Instant::now();

    (0..full_batches).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let mut local_fox_found = 0;

        for _ in 0..BATCH_SIZE {
            let mut board = Board::new();
            board.fill_and_shuffle(&mut rng);
            if board.check_for_word_fox() {
                local_fox_found += 1;
            }
        }

        fox_found.fetch_add(local_fox_found, Ordering::Relaxed);
    });

    if remainder > 0 {
        let mut rng = rand::thread_rng();
        let mut local_fox_found = 0;

        for _ in 0..remainder {
            let mut board = Board::new();
            board.fill_and_shuffle(&mut rng);
            if board.check_for_word_fox() {
                local_fox_found += 1;
            }
        }

        fox_found.fetch_add(local_fox_found, Ordering::Relaxed);
    }

    let total_time = start_time.elapsed();
    let total_seconds = total_time.as_secs_f64();
    let avg_time_per_simulation = total_seconds / num_of_games as f64;

    let fox_found = fox_found.load(Ordering::Relaxed);
    let fox_not_found = num_of_games - fox_found;

    println!(
        "Fox found: {} -> {}%\nFox not found: {} -> {}%",
        fox_found,
        (fox_found as f64 / num_of_games as f64) * 100.0,
        fox_not_found,
        (fox_not_found as f64 / num_of_games as f64) * 100.0
    );

    println!("Total time for {} games: {} seconds", num_of_games, total_seconds);
    println!("Average time per simulation: {} seconds", avg_time_per_simulation);
}
