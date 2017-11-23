extern crate snake;
extern crate rand;
extern crate clap;

use snake::game_loop::*;
use std::process;
use rand::thread_rng;
use clap::{Arg, App};

fn main() {
    let matches = App::new("snake")
        .version("0.0.2")
        .author("Adracus <adracus@gmail.com>")
        .about("The classic snake game - implemented in Rust")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .default_value("20")
            .help("Width of the game field")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .default_value("10")
            .help("Height of the game field")
            .takes_value(true))
        .get_matches();

    let width = matches.value_of("width");
    let height = matches.value_of("height");

    let width = width.unwrap().parse::<u8>().unwrap();
    let height = height.unwrap().parse::<u8>().unwrap();

    let rng = thread_rng();

    if let Err(e) = run_game(rng, width, height) {
        println!("Encountered error: {}", e);

        process::exit(1);
    }
}
