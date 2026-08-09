use crate::render::Renderer;
use retro_terminal::Terminal;

pub fn color_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("ANSI COLOR CELLS\n");
    terminal.feed("================\n");

    for code in 30..=37 {
        terminal.feed(&format!("\x1b[{code}m{code} \x1b[0m"));
    }

    terminal.feed("\n");

    for code in 90..=97 {
        terminal.feed(&format!("\x1b[{code}m{code} \x1b[0m"));
    }

    renderer.render(terminal);

    println!("Use :cell <row> <column> to inspect stored styles.");
}
