//! Core retro terminal primitives.

/// Supported high-level terminal actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalCommand {
    Text(String),
    NewLine,
    CarriageReturn,
    Backspace,
    Clear,
    MoveTo { row: usize, col: usize },
}

/// A single terminal cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ' }
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
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0, "terminal width must be greater than zero");
        assert!(height > 0, "terminal height must be greater than zero");

        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
            cursor: Cursor { row: 0, col: 0 },
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
        self.cursor = Cursor { row: 0, col: 0 };
    }

    pub fn execute(&mut self, command: TerminalCommand) {
        match command {
            TerminalCommand::Text(text) => self.write_text(&text),
            TerminalCommand::NewLine => self.newline(),
            TerminalCommand::CarriageReturn => self.cursor.col = 0,
            TerminalCommand::Backspace => self.backspace(),
            TerminalCommand::Clear => self.clear(),
            TerminalCommand::MoveTo { row, col } => self.move_to(row, col),
        }
    }

    pub fn feed(&mut self, input: &str) {
        for ch in input.chars() {
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
            _ if ch.is_control() => {}
            _ => self.put_printable(ch),
        }
    }

    fn put_printable(&mut self, ch: char) {
        if self.cursor.col >= self.width {
            self.newline();
        }

        if let Some(idx) = self.index(self.cursor.row, self.cursor.col) {
            self.cells[idx].ch = ch;
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
            if let Some(idx) = self.index(self.cursor.row, self.cursor.col) {
                self.cells[idx] = Cell::default();
            }
        }
    }

    fn move_to(&mut self, row: usize, col: usize) {
        self.cursor.row = row.min(self.height - 1);
        self.cursor.col = col.min(self.width - 1);
    }

    fn scroll_up(&mut self) {
        for row in 1..self.height {
            let src_start = row * self.width;
            let src_end = src_start + self.width;
            let dst_start = (row - 1) * self.width;
            self.cells.copy_within(src_start..src_end, dst_start);
        }

        let last_row_start = (self.height - 1) * self.width;
        self.cells[last_row_start..].fill(Cell::default());
        self.cursor.row = self.height - 1;
    }

    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.height && col < self.width {
            Some(row * self.width + col)
        } else {
            None
        }
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
        assert_eq!(terminal.lines(), vec!["HELL".to_string(), "O   ".to_string()]);
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
    fn supports_move_and_clear_commands() {
        let mut terminal = Terminal::new(4, 2);
        terminal.execute(TerminalCommand::Text("ABCD".to_string()));
        terminal.execute(TerminalCommand::MoveTo { row: 1, col: 1 });
        terminal.execute(TerminalCommand::Text("Z".to_string()));
        assert_eq!(terminal.lines(), vec!["ABCD".to_string(), " Z  ".to_string()]);

        terminal.execute(TerminalCommand::Clear);
        assert_eq!(terminal.lines(), vec!["    ".to_string(), "    ".to_string()]);
        assert_eq!(terminal.cursor(), Cursor { row: 0, col: 0 });
    }
}
