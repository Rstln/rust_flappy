use bracket_lib::prelude::*;
use state::State;

mod player;
mod state;
mod gamemode;
mod obstacle;
mod constants;


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("flappy")
        .build()?;
    main_loop(context, State::new())
}
