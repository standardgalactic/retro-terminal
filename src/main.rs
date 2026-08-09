use retro_terminal::{version, Terminal, TerminalCommand, ThemePreset};
use std::io::{self, Write};

const DEFAULT_WIDTH: usize = 40;
const DEFAULT_HEIGHT: usize = 12;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let mut state = AppState::default();

    banner();
    print_menu();
    render(&terminal);

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print_prompt(&state)?;

        line.clear();

        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let input = line.trim();

        if input.is_empty() {
            continue;
        }

        state.command_count += 1;
        state.history.push(input.to_string());

        let keep_running = if input.starts_with(':') {
            handle_command(input, &mut terminal, &mut state)?
        } else {
            terminal.feed(input);
            terminal.feed("\n");
            render(&terminal);
            true
        };

        if !keep_running {
            break;
        }
    }

    println!();
    println!("Session terminated.");
    println!("Commands entered: {}", state.command_count);
    println!("Goodbye!");

    Ok(())
}

struct AppState {
    theme_name: &'static str,
    command_count: usize,
    history: Vec<String>,
    prompt: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            // Terminal::new() starts with ThemePreset::Green.
            theme_name: "green",
            command_count: 0,
            history: Vec::new(),
            prompt: "retro".to_string(),
        }
    }
}

fn handle_command(
    input: &str,
    terminal: &mut Terminal,
    state: &mut AppState,
) -> io::Result<bool> {
    let mut parts = input.split_whitespace();

    let command = parts.next().unwrap_or("");

    match command {
        ":help" | ":?" => {
            print_help(parts.next());
        }

        ":menu" => {
            print_menu();
        }

        ":about" => {
            print_about();
        }

        ":version" => {
            println!("retro-terminal {}", version());
        }

        ":clear" | ":cls" => {
            terminal.clear();
            render(terminal);
        }

        ":clearline" => {
            terminal.execute(TerminalCommand::ClearLine);
            render(terminal);
        }

        ":render" | ":screen" => {
            render(terminal);
        }

        ":theme" => {
            if let Some(name) = parts.next() {
                set_theme(name, terminal, state);
            } else {
                print_theme_menu();
            }
        }

        ":themes" => {
            print_theme_menu();
        }

        ":cursor" => {
            handle_cursor(parts, terminal);
        }

        ":up" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveUp(amount));
            print_cursor(terminal);
            render(terminal);
        }

        ":down" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveDown(amount));
            print_cursor(terminal);
            render(terminal);
        }

        ":left" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveLeft(amount));
            print_cursor(terminal);
            render(terminal);
        }

        ":right" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveRight(amount));
            print_cursor(terminal);
            render(terminal);
        }

        ":status" => {
            print_status(terminal, state);
        }

        ":dimensions" => {
            let (width, height) = terminal.dimensions();
            println!("Terminal dimensions: {width} x {height}");
        }

        ":cell" => {
            inspect_cell(parts, terminal);
        }

        ":dump" => {
            dump_terminal(terminal);
        }

        ":history" => {
            match parts.next() {
                Some("clear") => {
                    state.history.clear();
                    println!("Command history cleared.");
                }
                Some(other) => {
                    println!("Unknown history command: {other}");
                    println!("Usage: :history [clear]");
                }
                None => print_history(state),
            }
        }

        ":system" | ":sysinfo" => {
            print_system_info(terminal, state);
        }

        ":demo" => {
            run_demo(terminal);
        }

        ":ansi" | ":ansitest" => {
            ansi_demo(terminal);
        }

        ":colors" => {
            color_demo(terminal);
        }

        ":cursortest" => {
            cursor_demo(terminal);
        }

        ":box" | ":charset" => {
            character_demo(terminal);
        }

        ":wraptest" => {
            wrap_demo(terminal);
        }

        ":scrolltest" => {
            scroll_demo(terminal);
        }

        ":matrix" => {
            matrix_demo(terminal);
        }

        ":boot" => {
            boot_demo(parts.next().unwrap_or("retro"), terminal);
        }

        ":test" => {
            terminal_test(terminal);
        }

        ":echo" => {
            let text = parts.collect::<Vec<_>>().join(" ");

            if text.is_empty() {
                println!("Usage: :echo <text>");
            } else {
                terminal.feed(&text);
                terminal.feed("\n");
                render(terminal);
            }
        }

        ":prompt" => {
            let new_prompt = parts.collect::<Vec<_>>().join(" ");

            if new_prompt.is_empty() {
                println!("Current prompt: {}>", state.prompt);
            } else {
                state.prompt = new_prompt;
                println!("Prompt changed to {}>", state.prompt);
            }
        }

        ":reset" => {
            let current_theme = terminal.theme();

            *terminal =
                Terminal::new_with_theme(DEFAULT_WIDTH, DEFAULT_HEIGHT, current_theme);

            state.history.clear();
            state.command_count = 0;

            println!("Terminal and session state reset.");
            render(terminal);
        }

        ":quit" | ":exit" | ":q" => {
            return Ok(false);
        }

        _ => {
            println!("Unknown command: {command}");
            println!("Type :help for available commands.");
        }
    }

    Ok(true)
}

fn parse_amount(value: Option<&str>) -> usize {
    value
        .and_then(|text| text.parse::<usize>().ok())
        .unwrap_or(1)
}

fn handle_cursor<'a, I>(mut parts: I, terminal: &mut Terminal)
where
    I: Iterator<Item = &'a str>,
{
    let row = parts.next();
    let col = parts.next();

    match (row, col) {
        (None, None) => {
            print_cursor(terminal);
        }

        (Some(row), Some(col)) => {
            let row = match row.parse::<usize>() {
                Ok(value) if value > 0 => value - 1,
                _ => {
                    println!("Row must be a positive integer.");
                    return;
                }
            };

            let col = match col.parse::<usize>() {
                Ok(value) if value > 0 => value - 1,
                _ => {
                    println!("Column must be a positive integer.");
                    return;
                }
            };

            terminal.execute(TerminalCommand::MoveTo { row, col });

            print_cursor(terminal);
            render(terminal);
        }

        _ => {
            println!("Usage: :cursor");
            println!("       :cursor <row> <column>");
        }
    }
}

fn print_cursor(terminal: &Terminal) {
    let cursor = terminal.cursor();

    println!(
        "Cursor: row {}, column {}",
        cursor.row + 1,
        cursor.col + 1
    );
}

fn inspect_cell<'a, I>(mut parts: I, terminal: &Terminal)
where
    I: Iterator<Item = &'a str>,
{
    let Some(row) = parts.next() else {
        println!("Usage: :cell <row> <column>");
        return;
    };

    let Some(col) = parts.next() else {
        println!("Usage: :cell <row> <column>");
        return;
    };

    let Ok(row) = row.parse::<usize>() else {
        println!("Invalid row.");
        return;
    };

    let Ok(col) = col.parse::<usize>() else {
        println!("Invalid column.");
        return;
    };

    if row == 0 || col == 0 {
        println!("Coordinates are one-based.");
        return;
    }

    match terminal.cell(row - 1, col - 1) {
        Some(cell) => {
            println!();
            println!("CELL INSPECTOR");
            println!("--------------");
            println!("Position:    {row},{col}");
            println!("Character:   {:?}", cell.ch);
            println!("Foreground:  {:?}", cell.style.foreground);
            println!("Background:  {:?}", cell.style.background);
            println!("Intense:     {}", cell.style.intense);
            println!();
        }

        None => {
            let (width, height) = terminal.dimensions();

            println!(
                "Cell {row},{col} is outside the {}x{} terminal.",
                width, height
            );
        }
    }
}

fn banner() {
    println!();
    println!("+--------------------------------------+");
    println!("|       R E T R O  T E R M I N A L    |");
    println!("|              L A B                   |");
    println!("+--------------------------------------+");
    println!();
}

fn print_prompt(state: &AppState) -> io::Result<()> {
    print!("{}> ", state.prompt);
    io::stdout().flush()
}

fn print_menu() {
    println!();
    println!("+--------------------------------------+");
    println!("|          RETRO TERMINAL LAB          |");
    println!("+--------------------------------------+");
    println!("| Terminal & screen commands   :help   |");
    println!("| Display presets              :themes |");
    println!("| ANSI laboratory              :ansi   |");
    println!("| Color laboratory             :colors |");
    println!("| Cursor laboratory        :cursortest |");
    println!("| Character display            :box    |");
    println!("| Historical boot gallery      :boot   |");
    println!("| Diagnostics                  :status |");
    println!("| About                        :about  |");
    println!("| Quit                         :quit   |");
    println!("+--------------------------------------+");
    println!();
}

fn print_help(topic: Option<&str>) {
    match topic {
        Some("ansi") => {
            println!();
            println!("ANSI / CSI");
            println!("----------");
            println!("  :ansi             Run ANSI/CSI demonstration");
            println!("  :colors           Run SGR color demonstration");
            println!("  :cursortest       Demonstrate cursor movement");
        }

        Some("cursor") => {
            println!();
            println!("CURSOR COMMANDS");
            println!("---------------");
            println!("  :cursor                    Show cursor");
            println!("  :cursor <row> <column>     Move cursor");
            println!("  :up [n]");
            println!("  :down [n]");
            println!("  :left [n]");
            println!("  :right [n]");
        }

        Some("themes") => {
            print_theme_menu();
        }

        Some("diagnostics") => {
            println!();
            println!("DIAGNOSTICS");
            println!("-----------");
            println!("  :status");
            println!("  :dimensions");
            println!("  :cell <row> <column>");
            println!("  :dump");
            println!("  :history");
            println!("  :system");
        }

        Some("tests") => {
            println!();
            println!("TESTS");
            println!("-----");
            println!("  :demo");
            println!("  :ansi");
            println!("  :colors");
            println!("  :cursortest");
            println!("  :charset");
            println!("  :wraptest");
            println!("  :scrolltest");
            println!("  :test");
        }

        Some(other) => {
            println!("Unknown help topic: {other}");
            println!(
                "Topics: ansi, cursor, themes, diagnostics, tests"
            );
        }

        None => {
            println!();
            println!("RETRO TERMINAL COMMANDS");
            println!("=======================");
            println!();

            println!("General");
            println!("  :menu");
            println!("  :help [topic]");
            println!("  :about");
            println!("  :version");
            println!("  :quit");
            println!();

            println!("Terminal");
            println!("  :clear");
            println!("  :clearline");
            println!("  :render");
            println!("  :echo <text>");
            println!("  :prompt <name>");
            println!("  :reset");
            println!();

            println!("Display");
            println!("  :themes");
            println!("  :theme <amber|green|ibmdos>");
            println!();

            println!("Cursor");
            println!("  :cursor [row column]");
            println!("  :up [n]");
            println!("  :down [n]");
            println!("  :left [n]");
            println!("  :right [n]");
            println!();

            println!("Laboratory");
            println!("  :ansi");
            println!("  :colors");
            println!("  :cursortest");
            println!("  :charset");
            println!("  :wraptest");
            println!("  :scrolltest");
            println!("  :matrix");
            println!("  :boot [retro|dos|unix|mainframe|vt100]");
            println!("  :test");
            println!();

            println!("Diagnostics");
            println!("  :status");
            println!("  :dimensions");
            println!("  :cell <row> <column>");
            println!("  :dump");
            println!("  :history [clear]");
            println!("  :system");
            println!();

            println!("Anything without ':' is written into the terminal.");
            println!();
        }
    }
}

fn print_theme_menu() {
    println!();
    println!("DISPLAY PRESETS");
    println!("---------------");
    println!("  amber    Amber phosphor");
    println!("  green    Green phosphor");
    println!("  ibmdos   IBM DOS inspired");
    println!();
    println!("Usage: :theme <name>");
}

fn set_theme(
    name: &str,
    terminal: &mut Terminal,
    state: &mut AppState,
) {
    let selected = match name.to_ascii_lowercase().as_str() {
        "amber" => Some((ThemePreset::Amber, "amber")),
        "green" => Some((ThemePreset::Green, "green")),
        "ibmdos" | "dos" | "ibm" => {
            Some((ThemePreset::IbmDos, "ibmdos"))
        }
        _ => None,
    };

    match selected {
        Some((theme, canonical_name)) => {
            terminal.set_theme(theme);
            state.theme_name = canonical_name;

            println!("Theme set to {canonical_name}.");
            render(terminal);
        }

        None => {
            println!("Unknown theme: {name}");
            print_theme_menu();
        }
    }
}

fn print_status(terminal: &Terminal, state: &AppState) {
    let (width, height) = terminal.dimensions();
    let cursor = terminal.cursor();
    let style = terminal.current_style();

    println!();
    println!("SESSION STATUS");
    println!("--------------");
    println!("Dimensions:        {width} x {height}");
    println!(
        "Cursor:            {},{}",
        cursor.row + 1,
        cursor.col + 1
    );
    println!("Theme:             {}", state.theme_name);
    println!("Foreground:        {:?}", style.foreground);
    println!("Background:        {:?}", style.background);
    println!("Intense:           {}", style.intense);
    println!("Commands entered:  {}", state.command_count);
    println!("History entries:   {}", state.history.len());
    println!("Prompt:            {}>", state.prompt);
    println!();
}

fn print_history(state: &AppState) {
    println!();
    println!("COMMAND HISTORY");
    println!("---------------");

    if state.history.is_empty() {
        println!("No commands entered.");
    } else {
        for (index, command) in state.history.iter().enumerate() {
            println!("{:>4}  {}", index + 1, command);
        }
    }

    println!();
}

fn dump_terminal(terminal: &Terminal) {
    println!();
    println!("TERMINAL BUFFER");
    println!("---------------");

    for (row, line) in terminal.lines().iter().enumerate() {
        println!("{:02} |{}|", row + 1, line);
    }

    println!();
}

fn print_about() {
    println!();
    println!("RETRO TERMINAL LAB");
    println!("==================");
    println!();
    println!("An interactive laboratory for exploring");
    println!("historical terminal behavior, ANSI control");
    println!("sequences, cursor movement, display styles,");
    println!("screen buffering, wrapping, and scrolling.");
    println!();
    println!("It also serves as an interactive test bench");
    println!("for the retro_terminal Rust library.");
    println!();
}

fn print_system_info(terminal: &Terminal, state: &AppState) {
    let (width, height) = terminal.dimensions();

    println!();
    println!("RETRO SYSTEM MONITOR");
    println!("--------------------");
    println!("SYSTEM       RETRO TERMINAL LAB");
    println!("VERSION      {}", version());
    println!("DISPLAY      {width} x {height}");
    println!("THEME        {}", state.theme_name.to_ascii_uppercase());
    println!("TERMINAL     READY");
    println!("STATUS       NOMINAL");
    println!();
}

fn run_demo(terminal: &mut Terminal) {
    terminal.clear();

    terminal.feed("RETRO TERMINAL LAB\n");
    terminal.feed("==================\n\n");
    terminal.feed("Text output........ OK\n");
    terminal.feed("Wrapping........... OK\n");
    terminal.feed("Scrolling.......... OK\n");
    terminal.feed("Cursor control..... OK\n");
    terminal.feed("ANSI CSI............ OK\n");
    terminal.feed("SGR styles.......... OK\n");

    render(terminal);
}

fn ansi_demo(terminal: &mut Terminal) {
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

    render(terminal);
}

fn color_demo(terminal: &mut Terminal) {
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

    render(terminal);

    println!("Use :cell <row> <column> to inspect stored styles.");
}

fn cursor_demo(terminal: &mut Terminal) {
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

    render(terminal);
    print_cursor(terminal);
}

fn character_demo(terminal: &mut Terminal) {
    terminal.clear();

    terminal.feed("CHARACTER TEST\n");
    terminal.feed("==============\n");
    terminal.feed("ABCDEFGHIJKLMNOPQRSTUVWXYZ\n");
    terminal.feed("abcdefghijklmnopqrstuvwxyz\n");
    terminal.feed("0123456789\n");
    terminal.feed("!@#$%^&*()_+-=[]{}\n");

    render(terminal);
}

fn wrap_demo(terminal: &mut Terminal) {
    terminal.clear();

    terminal.feed("WRAP TEST\n");
    terminal.feed("---------\n");

    terminal.feed(
        "This sentence is intentionally much longer than the forty-column terminal width so that wrapping is visible.",
    );

    render(terminal);
}

fn scroll_demo(terminal: &mut Terminal) {
    terminal.clear();

    for index in 1..=20 {
        terminal.feed(&format!("SCROLL LINE {index:02}\n"));
    }

    render(terminal);
}

fn matrix_demo(terminal: &mut Terminal) {
    terminal.clear();

    terminal.feed("01001000 01000001 01010000\n");
    terminal.feed("11001010 00110101 11100010\n");
    terminal.feed("00110110 10101010 01010101\n");
    terminal.feed("11110000 00001111 10100101\n");
    terminal.feed("01010101 11001100 00110011\n");
    terminal.feed("\nDATA STREAM SYNCHRONIZED\n");

    render(terminal);
}

fn boot_demo(profile: &str, terminal: &mut Terminal) {
    terminal.clear();

    match profile.to_ascii_lowercase().as_str() {
        "retro" => {
            terminal.feed("RETRO SYSTEM BIOS 0.1\n\n");
            terminal.feed("MEMORY CHECK........ OK\n");
            terminal.feed("DISPLAY............. OK\n");
            terminal.feed("ANSI DRIVER......... OK\n");
            terminal.feed("TERMINAL............ OK\n\n");
            terminal.feed("SYSTEM READY\n");
        }

        "dos" => {
            terminal.feed("RETRO PC BIOS\n\n");
            terminal.feed("640K RAM OK\n");
            terminal.feed("Loading operating system...\n\n");
            terminal.feed("C:\\>");
        }

        "unix" => {
            terminal.feed("Booting UNIX...\n");
            terminal.feed("memory: ok\n");
            terminal.feed("tty0: ready\n");
            terminal.feed("filesystem: mounted\n\n");
            terminal.feed("login: ");
        }

        "mainframe" => {
            terminal.feed("SYSTEM/370 CONSOLE\n");
            terminal.feed("------------------\n");
            terminal.feed("INITIAL PROGRAM LOAD\n");
            terminal.feed("CHANNELS ONLINE\n");
            terminal.feed("OPERATOR CONSOLE READY\n");
        }

        "vt100" => {
            terminal.feed("DEC VT100\n");
            terminal.feed("VIDEO TERMINAL ONLINE\n");
            terminal.feed("ANSI CONTROL MODE READY\n");
            terminal.feed("\nREADY.\n");
        }

        other => {
            println!("Unknown boot profile: {other}");
            println!("Profiles: retro, dos, unix, mainframe, vt100");
            return;
        }
    }

    render(terminal);
}

fn terminal_test(terminal: &mut Terminal) {
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

    render(terminal);
}

fn render(terminal: &Terminal) {
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