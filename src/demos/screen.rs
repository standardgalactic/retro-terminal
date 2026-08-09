use crate::render::Renderer;
use retro_terminal::Terminal;

pub fn run_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("RETRO TERMINAL LAB\n");
    terminal.feed("==================\n\n");
    terminal.feed("Text output........ OK\n");
    terminal.feed("Wrapping........... OK\n");
    terminal.feed("Scrolling.......... OK\n");
    terminal.feed("Cursor control..... OK\n");
    terminal.feed("ANSI CSI............ OK\n");
    terminal.feed("SGR styles.......... OK\n");

    renderer.render(terminal);
}

pub fn character_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("CHARACTER TEST\n");
    terminal.feed("==============\n");
    terminal.feed("ABCDEFGHIJKLMNOPQRSTUVWXYZ\n");
    terminal.feed("abcdefghijklmnopqrstuvwxyz\n");
    terminal.feed("0123456789\n");
    terminal.feed("!@#$%^&*()_+-=[]{}\n");

    renderer.render(terminal);
}

pub fn wrap_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("WRAP TEST\n");
    terminal.feed("---------\n");

    terminal.feed(
        "This sentence is intentionally much longer than the forty-column terminal width so that wrapping is visible.",
    );

    renderer.render(terminal);
}

pub fn scroll_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    for index in 1..=20 {
        terminal.feed(&format!("SCROLL LINE {index:02}\n"));
    }

    renderer.render(terminal);
}

pub fn matrix_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("01001000 01000001 01010000\n");
    terminal.feed("11001010 00110101 11100010\n");
    terminal.feed("00110110 10101010 01010101\n");
    terminal.feed("11110000 00001111 10100101\n");
    terminal.feed("01010101 11001100 00110011\n");
    terminal.feed("\nDATA STREAM SYNCHRONIZED\n");

    renderer.render(terminal);
}

pub fn terminal_test(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("0123456789012345678901234567890123456789\n");
    terminal.feed("ABCDEFGHIJKLMNOPQRSTUVWXYZ\n");
    terminal.feed("abcdefghijklmnopqrstuvwxyz\n");
    terminal.feed("----------------------------------------\n");
    terminal.feed("TAB:\tX\n");
    terminal.feed("RETURN TEST: ABC\rXYZ\n");
    terminal.feed("BACKSPACE: ABC\x08!\n");

    for index in 1..=10 {
        terminal.feed(&format!("SCROLL TEST {index:02}\n"));
    }

    renderer.render(terminal);
}
