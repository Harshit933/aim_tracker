mod config;
mod cube;
mod state;

use ggez::{event, GameResult};

use state::MainState;

fn main() -> GameResult {
    let conf = config::Config::new("config.toml");
    let cb = ggez::ContextBuilder::new("drawing", "ggez").window_mode(
        ggez::conf::WindowMode::default().dimensions(conf.width as f32, conf.height as f32),
    );
    let (mut ctx, events_loop) = cb.build()?;

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
