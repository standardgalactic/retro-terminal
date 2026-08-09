use retro_terminal::{Color, Terminal, TextStyle};

pub trait Renderer {
    fn render(&self, terminal: &Terminal);
}

#[derive(Default)]
pub struct PlainRenderer;

impl Renderer for PlainRenderer {
    fn render(&self, terminal: &Terminal) {
        let (width, _) = terminal.dimensions();

        println!();

        let border = format!("+{}+", "-".repeat(width + 2));
        println!("{border}");

        for line in terminal.lines() {
            println!("| {line} |");
        }

        println!("{border}");

        let cursor = terminal.cursor();

        println!(
            " {}x{}  cursor {},{}  {:?}",
            terminal.dimensions().0,
            terminal.dimensions().1,
            cursor.row + 1,
            cursor.col + 1,
            terminal.theme()
        );

        println!();
    }
}

#[derive(Default)]
pub struct AnsiRenderer;

impl Renderer for AnsiRenderer {
    fn render(&self, terminal: &Terminal) {
        let (width, height) = terminal.dimensions();

        println!();

        let border = format!("+{}+", "-".repeat(width + 2));
        println!("{border}");

        for row in 0..height {
            print!("| ");

            let mut current_style: Option<TextStyle> = None;

            for col in 0..width {
                if let Some(cell) = terminal.cell(row, col) {
                    if current_style != Some(cell.style) {
                        print!("{}", style_sequence(cell.style));
                        current_style = Some(cell.style);
                    }

                    print!("{}", cell.ch);
                }
            }

            if current_style.is_some() {
                print!("\x1b[0m");
            }

            println!(" |");
        }

        println!("{border}");

        let cursor = terminal.cursor();

        println!(
            " {}x{}  cursor {},{}  {:?}",
            terminal.dimensions().0,
            terminal.dimensions().1,
            cursor.row + 1,
            cursor.col + 1,
            terminal.theme()
        );

        println!();
    }
}

fn style_sequence(style: TextStyle) -> String {
    let intense = if style.intense { 1 } else { 22 };
    format!(
        "\x1b[{intense};{};{}m",
        foreground_code(style.foreground),
        background_code(style.background)
    )
}

fn foreground_code(color: Color) -> u8 {
    match color {
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::White => 37,
        Color::BrightBlack => 90,
        Color::BrightRed => 91,
        Color::BrightGreen => 92,
        Color::BrightYellow => 93,
        Color::BrightBlue => 94,
        Color::BrightMagenta => 95,
        Color::BrightCyan => 96,
        Color::BrightWhite => 97,
    }
}

fn background_code(color: Color) -> u8 {
    match color {
        Color::Black => 40,
        Color::Red => 41,
        Color::Green => 42,
        Color::Yellow => 43,
        Color::Blue => 44,
        Color::Magenta => 45,
        Color::Cyan => 46,
        Color::White => 47,
        Color::BrightBlack => 100,
        Color::BrightRed => 101,
        Color::BrightGreen => 102,
        Color::BrightYellow => 103,
        Color::BrightBlue => 104,
        Color::BrightMagenta => 105,
        Color::BrightCyan => 106,
        Color::BrightWhite => 107,
    }
}
