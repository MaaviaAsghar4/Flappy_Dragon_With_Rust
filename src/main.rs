#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

mod constants;
mod obstacle;
mod player;
mod state;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(
            constants::SCREEN_WIDTH,
            constants::SCREEN_HEIGHT,
            "../resources/flappy32.png",
        )
        .with_fancy_console(
            constants::SCREEN_WIDTH,
            constants::SCREEN_HEIGHT,
            "../resources/flappy32.png",
        )
        .with_title("Flappy Dragon")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, state::State::new())
}
