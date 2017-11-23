extern crate ncurses;

use rand::Rng;
use logic::*;
use geometry::*;
use renderer::*;
use std::{time, thread};

const MS_PER_FRAME: u64 = 300;

pub fn run_game<'a, R: Rng>(rng: R, width: u8, height: u8) -> Result<(), &'a str> {
    let bounds = Bounds::new(width, height);

    if bounds.area() < 16 {
        return Err("Too small area to play with");
    }

    let mut renderer = NCurses::new();
    let init_game = NonInitializedGame::new(rng, bounds);

    renderer.render(&init_game);

    let direction = renderer.get_direction().unwrap_or(Direction::Down);

    let mut game = init_game.start(direction);

    loop {
        if let Some(new_direction) = renderer.get_direction_nonblocking() {
            game.change_direction(new_direction);
        }

        let start = time::Instant::now();
        let state = game.tick();

        renderer.render(&game);

        match state {
            GameResult::Ongoing => (),
            GameResult::Won => {
                renderer.render_message("You won!");
                break
            },
            GameResult::Lost => {
                renderer.render_message("You lost!");
                break
            },
        }

        let elapsed = start.elapsed();
        let to_sleep = time::Duration::from_millis(MS_PER_FRAME) - elapsed;
        thread::sleep(to_sleep);
    }

    renderer.wait_for_confirmation();
    
    Ok(())
}

