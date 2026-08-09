mod app;
mod commands;
mod demos;
mod render;

use app::{App, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use render::{AnsiRenderer, PlainRenderer, Renderer};
use retro_terminal::Terminal;
use std::io;

fn main() -> io::Result<()> {
    let terminal = Terminal::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let renderer: Box<dyn Renderer> = match std::env::var("RETRO_RENDERER").ok().as_deref() {
        Some("ansi") => Box::new(AnsiRenderer),
        _ => Box::new(PlainRenderer),
    };
    let mut app = App::with_renderer(terminal, renderer);
    app.run()
}
