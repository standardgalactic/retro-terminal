use crate::app::{AppState, DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::demos;
use crate::render::Renderer;
use retro_terminal::{version, Terminal, TerminalCommand, ThemePreset};
use std::io;

pub fn handle_command(
    input: &str,
    terminal: &mut Terminal,
    state: &mut AppState,
    renderer: &dyn Renderer,
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
            renderer.render(terminal);
        }

        ":clearline" => {
            terminal.execute(TerminalCommand::ClearLine);
            renderer.render(terminal);
        }

        ":render" | ":screen" => {
            renderer.render(terminal);
        }

        ":theme" => {
            if let Some(name) = parts.next() {
                set_theme(name, terminal, state, renderer);
            } else {
                print_theme_menu();
            }
        }

        ":themes" => {
            print_theme_menu();
        }

        ":cursor" => {
            handle_cursor(parts, terminal, renderer);
        }

        ":up" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveUp(amount));
            print_cursor(terminal);
            renderer.render(terminal);
        }

        ":down" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveDown(amount));
            print_cursor(terminal);
            renderer.render(terminal);
        }

        ":left" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveLeft(amount));
            print_cursor(terminal);
            renderer.render(terminal);
        }

        ":right" => {
            let amount = parse_amount(parts.next());
            terminal.execute(TerminalCommand::MoveRight(amount));
            print_cursor(terminal);
            renderer.render(terminal);
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

        ":history" => match parts.next() {
            Some("clear") => {
                state.history.clear();
                println!("Command history cleared.");
            }
            Some(other) => {
                println!("Unknown history command: {other}");
                println!("Usage: :history [clear]");
            }
            None => print_history(state),
        },

        ":system" | ":sysinfo" => {
            print_system_info(terminal, state);
        }

        ":demo" => {
            demos::run_demo(terminal, renderer);
        }

        ":ansi" | ":ansitest" => {
            demos::ansi_demo(terminal, renderer);
        }

        ":colors" => {
            demos::color_demo(terminal, renderer);
        }

        ":cursortest" => {
            demos::cursor_demo(terminal, renderer);
        }

        ":box" | ":charset" => {
            demos::character_demo(terminal, renderer);
        }

        ":wraptest" => {
            demos::wrap_demo(terminal, renderer);
        }

        ":scrolltest" => {
            demos::scroll_demo(terminal, renderer);
        }

        ":matrix" => {
            demos::matrix_demo(terminal, renderer);
        }

        ":boot" => {
            demos::boot_demo(parts.next().unwrap_or("retro"), terminal, renderer);
        }

        ":test" => {
            demos::terminal_test(terminal, renderer);
        }

        ":echo" => {
            let text = parts.collect::<Vec<_>>().join(" ");

            if text.is_empty() {
                println!("Usage: :echo <text>");
            } else {
                terminal.feed(&text);
                terminal.feed("\n");
                renderer.render(terminal);
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

            *terminal = Terminal::new_with_theme(DEFAULT_WIDTH, DEFAULT_HEIGHT, current_theme);

            state.history.clear();
            state.command_count = 0;

            println!("Terminal and session state reset.");
            renderer.render(terminal);
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

fn handle_cursor<'a, I>(mut parts: I, terminal: &mut Terminal, renderer: &dyn Renderer)
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
            renderer.render(terminal);
        }

        _ => {
            println!("Usage: :cursor");
            println!("       :cursor <row> <column>");
        }
    }
}

fn print_cursor(terminal: &Terminal) {
    let cursor = terminal.cursor();

    println!("Cursor: row {}, column {}", cursor.row + 1, cursor.col + 1);
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

pub fn print_menu() {
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
            println!("Topics: ansi, cursor, themes, diagnostics, tests");
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

fn set_theme(name: &str, terminal: &mut Terminal, state: &mut AppState, renderer: &dyn Renderer) {
    let selected = match name.to_ascii_lowercase().as_str() {
        "amber" => Some((ThemePreset::Amber, "amber")),
        "green" => Some((ThemePreset::Green, "green")),
        "ibmdos" | "dos" | "ibm" => Some((ThemePreset::IbmDos, "ibmdos")),
        _ => None,
    };

    match selected {
        Some((theme, canonical_name)) => {
            terminal.set_theme(theme);
            state.theme_name = canonical_name;

            println!("Theme set to {canonical_name}.");
            renderer.render(terminal);
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
    println!("Cursor:            {},{}", cursor.row + 1, cursor.col + 1);
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
