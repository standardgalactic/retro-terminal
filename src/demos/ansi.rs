use crate::render::Renderer;
use retro_terminal::Terminal;

pub fn ansi_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("ANSI / CSI TEST\n");
    terminal.feed("===============\n");
    terminal.feed("HELLO");

    terminal.feed("\x1b[2D");
    terminal.feed("XX");

    terminal.feed("\x1b[3;5H");
    terminal.feed("POSITION");

    terminal.feed("\x1b[5;1H");
    terminal.feed("\x1b[1mINTENSE\x1b[0m");

    terminal.feed("\x1b[6;1H");
    terminal.feed("\x1b[31mRED\x1b[0m");

    renderer.render(terminal);
}
