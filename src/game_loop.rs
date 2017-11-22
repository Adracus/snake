extern crate ncurses;

use rand::Rng;
use logic::*;
use geometry::*;
use std::error::Error;
use renderer::*;
use std::{time, thread};

const MS_PER_FRAME: u64 = 300;

pub fn run_game<R: Rng>(rng: R) -> Result<(), Box<Error>> {
    let mut renderer = NCurses::new();
    let init_game = NonInitializedGame::new(rng, Bounds::new(10, 10));

    renderer.render(&init_game);

    let direction = renderer.get_direction().unwrap_or(Direction::Down);

    let mut game = init_game.start(direction);

    loop {
        game.direction =
            renderer.get_direction_nonblocking()
            .unwrap_or(game.direction);
        let start = time::Instant::now();
        let ok = game.tick();

        renderer.render(&game);
        
        if !ok {
           break;
        }

        let elapsed = start.elapsed();
        let to_sleep = time::Duration::from_millis(MS_PER_FRAME) - elapsed;
        thread::sleep(to_sleep);
    }
    
    Ok(())
}

