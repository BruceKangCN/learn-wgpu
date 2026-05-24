use anyhow::{Context, Result};
use tracing_subscriber::EnvFilter;
use winit::event_loop::EventLoop;

use crate::app::App;

mod app;
mod texture;
mod util;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let event_loop = EventLoop::with_user_event()
        .build()
        .context("failed to build event loop")?;
    let mut app = App::default();

    Ok(event_loop.run_app(&mut app)?)
}
