use retro_terminal::{Terminal, ThemePreset};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(40, 12);

    println!("Retro Terminal Demo");
    println!("Type text and press Enter to render.");
    println!("Commands: :help, :clear, :theme <amber|green|ibmdos>, :quit");
    render(&terminal);

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("retro> ");
        io::stdout().flush()?;

        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let input = line.trim_end();
        if input.is_empty() {
            continue;
        }

        if input == ":quit" {
            break;
        } else if input == ":help" {
            println!(":help                       Show this help");
            println!(":clear                      Clear the terminal buffer");
            println!(":theme amber|green|ibmdos   Change terminal theme");
            println!(":quit                       Exit the demo");
            continue;
        } else if input == ":clear" {
            terminal.clear();
            render(&terminal);
            continue;
        } else if let Some(theme_name) = input.strip_prefix(":theme ") {
            let theme = match theme_name.trim() {
                "amber" => Some(ThemePreset::Amber),
                "green" => Some(ThemePreset::Green),
                "ibmdos" => Some(ThemePreset::IbmDos),
                _ => None,
            };

            if let Some(theme) = theme {
                terminal.set_theme(theme);
                println!("Theme set to {}", theme_name.trim());
                render(&terminal);
            } else {
                println!("Unknown theme: {}", theme_name.trim());
            }
            continue;
        }

        terminal.feed(input);
        terminal.feed("\n");
        render(&terminal);
    }

    println!("Goodbye!");
    Ok(())
}

fn render(terminal: &Terminal) {
    println!();
    for line in terminal.lines() {
        println!("|{}|", line);
    }
    println!();
}
