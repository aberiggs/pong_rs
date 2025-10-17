mod game;

use ggez::{
    ContextBuilder, GameResult,
    event::{self},
};

fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Rusty Pong", "azriv")
        .build()
        .expect("Could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let state = game::GameState::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, state);
}
