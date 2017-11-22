extern crate snake;
extern crate rand;

use snake::game_loop::*;
use std::process;
use rand::thread_rng;

fn main() {
    let rng = thread_rng();

    if let Err(e) = run_game(rng) {
        println!("Encountered error: {}", e);

        process::exit(1);
    }
}
