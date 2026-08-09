use crate::render::Renderer;
use retro_terminal::{Terminal, TerminalCommand};

pub fn cursor_demo(terminal: &mut Terminal, renderer: &dyn Renderer) {
    terminal.clear();

    terminal.feed("CURSOR LAB\n");
    terminal.feed("----------\n");
    terminal.feed("AAAAAA\n");
    terminal.feed("BBBBBB\n");
    terminal.feed("CCCCCC");

    terminal.execute(TerminalCommand::MoveTo { row: 2, col: 2 });
    terminal.execute(TerminalCommand::Text("X".to_string()));

    terminal.execute(TerminalCommand::MoveTo { row: 3, col: 4 });
    terminal.execute(TerminalCommand::Text("Y".to_string()));

    renderer.render(terminal);
    print_cursor(terminal);
}

fn print_cursor(terminal: &Terminal) {
    let cursor = terminal.cursor();

    println!("Cursor: row {}, column {}", cursor.row + 1, cursor.col + 1);
}
