//! Core retro terminal primitives.

/// Supported high-level terminal actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalCommand {
    Text(String),
    NewLine,
    CarriageReturn,
    Backspace,
    Clear,
    ClearLine,
    MoveTo { row: usize, col: usize },
    MoveUp(usize),
    MoveDown(usize),
    MoveLeft(usize),
    MoveRight(usize),
    SetTheme(ThemePreset),
}

/// A single terminal cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub style: TextStyle,
}

/// Supported terminal color values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Style metadata associated with rendered text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyle {
    pub foreground: Color,
    pub background: Color,
    pub intense: bool,
}

impl TextStyle {
    fn from_theme(theme: ThemePreset) -> Self {
        theme.default_style()
    }
}

/// Visual preset inspired by historical terminal defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    Amber,
    Green,
    IbmDos,
}

impl ThemePreset {
    fn default_style(self) -> TextStyle {
        match self {
            ThemePreset::Amber => TextStyle {
                foreground: Color::Yellow,
                background: Color::Black,
                intense: true,
            },
            ThemePreset::Green => TextStyle {
                foreground: Color::Green,
                background: Color::Black,
                intense: true,
            },
            ThemePreset::IbmDos => TextStyle {
                foreground: Color::BrightWhite,
                background: Color::Blue,
                intense: false,
            },
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: ThemePreset::Green.default_style(),
        }
    }
}

/// Cursor position in the terminal buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

/// Fixed-size text terminal with wrap and scroll behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terminal {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    cursor: Cursor,
    theme: ThemePreset,
    current_style: TextStyle,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_theme(width, height, ThemePreset::Green)
    }

    pub fn new_with_theme(width: usize, height: usize, theme: ThemePreset) -> Self {
        assert!(width > 0, "terminal width must be greater than zero");
        assert!(height > 0, "terminal height must be greater than zero");
        let default_style = theme.default_style();

        Self {
            width,
            height,
            cells: vec![
                Cell {
                    ch: ' ',
                    style: default_style,
                };
                width * height
            ],
            cursor: Cursor { row: 0, col: 0 },
            theme,
            current_style: TextStyle::from_theme(theme),
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn theme(&self) -> ThemePreset {
        self.theme
    }

    pub fn current_style(&self) -> TextStyle {
        self.current_style
    }

    pub fn set_theme(&mut self, theme: ThemePreset) {
        self.theme = theme;
        self.current_style = theme.default_style();
    }

    pub fn clear(&mut self) {
        let blank = self.blank_cell();
        self.cells.fill(blank);
        self.cursor = Cursor { row: 0, col: 0 };
    }

    pub fn execute(&mut self, command: TerminalCommand) {
        match command {
            TerminalCommand::Text(text) => self.write_text(&text),
            TerminalCommand::NewLine => self.newline(),
            TerminalCommand::CarriageReturn => self.cursor.col = 0,
            TerminalCommand::Backspace => self.backspace(),
            TerminalCommand::Clear => self.clear(),
            TerminalCommand::ClearLine => self.clear_line(),
            TerminalCommand::MoveTo { row, col } => self.move_to(row, col),
            TerminalCommand::MoveUp(amount) => self.move_up(amount),
            TerminalCommand::MoveDown(amount) => self.move_down(amount),
            TerminalCommand::MoveLeft(amount) => self.move_left(amount),
            TerminalCommand::MoveRight(amount) => self.move_right(amount),
            TerminalCommand::SetTheme(theme) => self.set_theme(theme),
        }
    }

    pub fn feed(&mut self, input: &str) {
        let mut chars = input.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' && matches!(chars.peek(), Some('[')) {
                chars.next();
                self.consume_csi(&mut chars);
                continue;
            }

            self.write_char(ch);
        }
    }

    pub fn write_text(&mut self, text: &str) {
        for ch in text.chars() {
            self.write_char(ch);
        }
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<Cell> {
        self.index(row, col).map(|idx| self.cells[idx])
    }

    pub fn line(&self, row: usize) -> Option<String> {
        if row >= self.height {
            return None;
        }

        let start = row * self.width;
        let end = start + self.width;
        Some(self.cells[start..end].iter().map(|cell| cell.ch).collect())
    }

    pub fn lines(&self) -> Vec<String> {
        (0..self.height).filter_map(|row| self.line(row)).collect()
    }

    fn write_char(&mut self, ch: char) {
        match ch {
            '\n' => self.newline(),
            '\r' => self.cursor.col = 0,
            '\x08' => self.backspace(),
            '\x0c' => self.clear(),
            '\t' => self.tab(),
            _ if ch.is_control() => {}
            _ => self.put_printable(ch),
        }
    }

    fn put_printable(&mut self, ch: char) {
        if self.cursor.col >= self.width {
            self.newline();
        }

        if let Some(idx) = self.index(self.cursor.row, self.cursor.col) {
            self.cells[idx] = Cell {
                ch,
                style: self.current_style,
            };
            self.cursor.col += 1;
        }
    }

    fn newline(&mut self) {
        self.cursor.col = 0;
        if self.cursor.row + 1 >= self.height {
            self.scroll_up();
        } else {
            self.cursor.row += 1;
        }
    }

    fn backspace(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
            self.cursor.col = self.width - 1;
        } else {
            return;
        }

        if let Some(idx) = self.index(self.cursor.row, self.cursor.col) {
            self.cells[idx] = self.blank_cell();
        }
    }

    fn move_to(&mut self, row: usize, col: usize) {
        self.cursor.row = row.min(self.height - 1);
        self.cursor.col = col.min(self.width - 1);
    }

    fn move_up(&mut self, amount: usize) {
        self.cursor.row = self.cursor.row.saturating_sub(amount);
    }

    fn move_down(&mut self, amount: usize) {
        self.cursor.row = self.cursor.row.saturating_add(amount).min(self.height - 1);
    }

    fn move_left(&mut self, amount: usize) {
        self.cursor.col = self.cursor.col.saturating_sub(amount);
    }

    fn move_right(&mut self, amount: usize) {
        self.cursor.col = self.cursor.col.saturating_add(amount).min(self.width - 1);
    }

    fn clear_line(&mut self) {
        let row_start = self.cursor.row * self.width;
        let blank = self.blank_cell();
        self.cells[row_start..row_start + self.width].fill(blank);
        self.cursor.col = 0;
    }

    fn clear_line_from_cursor(&mut self) {
        if let Some(start) = self.index(self.cursor.row, self.cursor.col) {
            let row_end = (self.cursor.row + 1) * self.width;
            let blank = self.blank_cell();
            self.cells[start..row_end].fill(blank);
        }
    }

    fn tab(&mut self) {
        let spaces = 8 - (self.cursor.col % 8);
        for _ in 0..spaces {
            self.put_printable(' ');
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..self.height {
            let src_start = row * self.width;
            let src_end = src_start + self.width;
            let dst_start = (row - 1) * self.width;
            self.cells.copy_within(src_start..src_end, dst_start);
        }

        let last_row_start = (self.height - 1) * self.width;
        let blank = self.blank_cell();
        self.cells[last_row_start..].fill(blank);
        self.cursor.row = self.height - 1;
    }

    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.height && col < self.width {
            Some(row * self.width + col)
        } else {
            None
        }
    }

    fn consume_csi<I>(&mut self, chars: &mut std::iter::Peekable<I>)
    where
        I: Iterator<Item = char>,
    {
        let mut params = String::new();
        let mut final_byte = None;

        for ch in chars.by_ref() {
            if ch.is_ascii_alphabetic() {
                final_byte = Some(ch);
                break;
            }

            if ch.is_ascii_digit() || ch == ';' {
                params.push(ch);
            } else {
                return;
            }
        }

        let Some(final_byte) = final_byte else {
            return;
        };

        let parsed_params = parse_csi_params(&params);
        self.apply_csi(final_byte, &parsed_params);
    }

    fn apply_csi(&mut self, final_byte: char, params: &[usize]) {
        match final_byte {
            'A' => self.move_up(first_or_default(params, 1)),
            'B' => self.move_down(first_or_default(params, 1)),
            'C' => self.move_right(first_or_default(params, 1)),
            'D' => self.move_left(first_or_default(params, 1)),
            'H' | 'f' => {
                let row = first_or_default(params, 1).saturating_sub(1);
                let col = params.get(1).copied().unwrap_or(1).saturating_sub(1);
                self.move_to(row, col);
            }
            'J' => {
                let mode = first_or_default(params, 0);
                if mode == 2 || mode == 3 {
                    self.clear();
                }
            }
            'K' => {
                let mode = first_or_default(params, 0);
                if mode == 2 {
                    self.clear_line();
                } else {
                    self.clear_line_from_cursor();
                }
            }
            'm' => self.apply_sgr(params),
            _ => {}
        }
    }

    fn apply_sgr(&mut self, params: &[usize]) {
        if params.is_empty() {
            self.current_style = self.theme.default_style();
            return;
        }

        for param in params {
            match *param {
                0 => self.current_style = self.theme.default_style(),
                1 => self.current_style.intense = true,
                22 => self.current_style.intense = false,
                30..=37 => self.current_style.foreground = ansi_basic_color(*param - 30, false),
                39 => self.current_style.foreground = self.theme.default_style().foreground,
                40..=47 => self.current_style.background = ansi_basic_color(*param - 40, false),
                49 => self.current_style.background = self.theme.default_style().background,
                90..=97 => self.current_style.foreground = ansi_basic_color(*param - 90, true),
                100..=107 => self.current_style.background = ansi_basic_color(*param - 100, true),
                _ => {}
            }
        }
    }

    fn blank_cell(&self) -> Cell {
        Cell {
            ch: ' ',
            style: self.theme.default_style(),
        }
    }
}

fn parse_csi_params(params: &str) -> Vec<usize> {
    if params.is_empty() {
        return vec![];
    }

    params
        .split(';')
        .map(|part| {
            if part.is_empty() {
                0
            } else {
                part.parse::<usize>().unwrap_or(0)
            }
        })
        .collect()
}

fn first_or_default(params: &[usize], default: usize) -> usize {
    params
        .first()
        .copied()
        .filter(|value| *value != 0)
        .unwrap_or(default)
}

fn ansi_basic_color(index: usize, bright: bool) -> Color {
    match (index, bright) {
        (0, false) => Color::Black,
        (1, false) => Color::Red,
        (2, false) => Color::Green,
        (3, false) => Color::Yellow,
        (4, false) => Color::Blue,
        (5, false) => Color::Magenta,
        (6, false) => Color::Cyan,
        (7, false) => Color::White,
        (0, true) => Color::BrightBlack,
        (1, true) => Color::BrightRed,
        (2, true) => Color::BrightGreen,
        (3, true) => Color::BrightYellow,
        (4, true) => Color::BrightBlue,
        (5, true) => Color::BrightMagenta,
        (6, true) => Color::BrightCyan,
        _ => Color::BrightWhite,
    }
}

pub fn version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(version(), "0.1.0");
    }

    #[test]
    fn wraps_text_to_next_line() {
        let mut terminal = Terminal::new(4, 2);
        terminal.write_text("HELLO");
        assert_eq!(
            terminal.lines(),
            vec!["HELL".to_string(), "O   ".to_string()]
        );
    }

    #[test]
    fn scrolls_when_writing_past_last_line() {
        let mut terminal = Terminal::new(3, 2);
        terminal.write_text("abc\ndef\nghi");
        assert_eq!(terminal.lines(), vec!["def".to_string(), "ghi".to_string()]);
    }

    #[test]
    fn handles_carriage_return_and_backspace() {
        let mut terminal = Terminal::new(5, 1);
        terminal.feed("hello\rY");
        assert_eq!(terminal.lines(), vec!["Yello".to_string()]);

        terminal.feed("\x08!");
        assert_eq!(terminal.lines(), vec!["!ello".to_string()]);
    }

    #[test]
    fn backspace_crosses_lines() {
        let mut terminal = Terminal::new(4, 2);
        terminal.feed("ABCD");
        terminal.feed("E");
        terminal.feed("\x08\x08");
        assert_eq!(
            terminal.lines(),
            vec!["ABC ".to_string(), "    ".to_string()]
        );
        assert_eq!(terminal.cursor(), Cursor { row: 0, col: 3 });
    }

    #[test]
    fn tab_expands_to_next_tab_stop() {
        let mut terminal = Terminal::new(10, 1);
        terminal.feed("A\tB");
        assert_eq!(terminal.lines(), vec!["A       B ".to_string()]);
    }

    #[test]
    fn supports_extended_cursor_commands() {
        let mut terminal = Terminal::new(6, 2);
        terminal.execute(TerminalCommand::Text("abcdef".to_string()));
        terminal.execute(TerminalCommand::MoveTo { row: 1, col: 0 });
        terminal.execute(TerminalCommand::MoveRight(2));
        terminal.execute(TerminalCommand::Text("Z".to_string()));
        assert_eq!(
            terminal.lines(),
            vec!["abcdef".to_string(), "  Z   ".to_string()]
        );

        terminal.execute(TerminalCommand::MoveUp(1));
        terminal.execute(TerminalCommand::MoveLeft(1));
        terminal.execute(TerminalCommand::Text("Q".to_string()));
        assert_eq!(
            terminal.lines(),
            vec!["abQdef".to_string(), "  Z   ".to_string()]
        );
    }

    #[test]
    fn supports_clear_line_command() {
        let mut terminal = Terminal::new(5, 2);
        terminal.feed("hello\nworld");
        terminal.execute(TerminalCommand::ClearLine);
        assert_eq!(
            terminal.lines(),
            vec!["hello".to_string(), "     ".to_string()]
        );
        assert_eq!(terminal.cursor(), Cursor { row: 1, col: 0 });
    }

    #[test]
    fn parses_basic_ansi_csi_sequences() {
        let mut terminal = Terminal::new(8, 2);
        terminal.feed("hello");
        terminal.feed("\x1b[2D");
        terminal.feed("X");
        assert_eq!(
            terminal.lines(),
            vec!["helXo   ".to_string(), "        ".to_string()]
        );

        terminal.feed("\x1b[2;3H");
        terminal.feed("Z");
        assert_eq!(
            terminal.lines(),
            vec!["helXo   ".to_string(), "  Z     ".to_string()]
        );

        terminal.feed("\x1b[2K");
        assert_eq!(
            terminal.lines(),
            vec!["helXo   ".to_string(), "        ".to_string()]
        );

        terminal.feed("abc");
        terminal.feed("\x1b[2J");
        assert_eq!(
            terminal.lines(),
            vec!["        ".to_string(), "        ".to_string()]
        );
        assert_eq!(terminal.cursor(), Cursor { row: 0, col: 0 });
    }

    #[test]
    fn supports_move_and_clear_commands() {
        let mut terminal = Terminal::new(4, 2);
        terminal.execute(TerminalCommand::Text("ABCD".to_string()));
        terminal.execute(TerminalCommand::MoveTo { row: 1, col: 1 });
        terminal.execute(TerminalCommand::Text("Z".to_string()));
        assert_eq!(
            terminal.lines(),
            vec!["ABCD".to_string(), " Z  ".to_string()]
        );

        terminal.execute(TerminalCommand::Clear);
        assert_eq!(
            terminal.lines(),
            vec!["    ".to_string(), "    ".to_string()]
        );
        assert_eq!(terminal.cursor(), Cursor { row: 0, col: 0 });
    }

    #[test]
    fn supports_theme_presets() {
        let mut terminal = Terminal::new_with_theme(2, 1, ThemePreset::Amber);
        terminal.feed("A");
        let amber_cell = terminal.cell(0, 0).expect("cell present");
        assert_eq!(amber_cell.style.foreground, Color::Yellow);
        assert_eq!(amber_cell.style.background, Color::Black);
        assert!(amber_cell.style.intense);

        terminal.execute(TerminalCommand::SetTheme(ThemePreset::IbmDos));
        terminal.feed("B");
        let ibm_cell = terminal.cell(0, 1).expect("cell present");
        assert_eq!(ibm_cell.style.foreground, Color::BrightWhite);
        assert_eq!(ibm_cell.style.background, Color::Blue);
        assert!(!ibm_cell.style.intense);
    }

    #[test]
    fn parses_sgr_color_and_reset_sequences() {
        let mut terminal = Terminal::new(4, 1);
        terminal.feed("\x1b[31mR\x1b[44mB\x1b[0mN");

        let red = terminal.cell(0, 0).expect("cell present");
        assert_eq!(red.ch, 'R');
        assert_eq!(red.style.foreground, Color::Red);
        assert_eq!(red.style.background, Color::Black);

        let blue_bg = terminal.cell(0, 1).expect("cell present");
        assert_eq!(blue_bg.ch, 'B');
        assert_eq!(blue_bg.style.foreground, Color::Red);
        assert_eq!(blue_bg.style.background, Color::Blue);

        let reset = terminal.cell(0, 2).expect("cell present");
        assert_eq!(reset.ch, 'N');
        assert_eq!(reset.style.foreground, Color::Green);
        assert_eq!(reset.style.background, Color::Black);
    }
}
