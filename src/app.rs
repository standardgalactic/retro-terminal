use crate::commands;
use crate::render::Renderer;
use retro_terminal::Terminal;
use std::io::{self, Write};

pub const DEFAULT_WIDTH: usize = 40;
pub const DEFAULT_HEIGHT: usize = 12;

pub struct AppState {
    pub(crate) theme_name: &'static str,
    pub(crate) command_count: usize,
    pub(crate) history: Vec<String>,
    pub(crate) prompt: String,
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

pub struct App {
    terminal: Terminal,
    state: AppState,
    renderer: Box<dyn Renderer>,
}

impl App {
    pub fn with_renderer(terminal: Terminal, renderer: Box<dyn Renderer>) -> Self {
        Self {
            terminal,
            state: AppState::default(),
            renderer,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        banner();
        commands::print_menu();
        self.renderer.render(&self.terminal);

        let stdin = io::stdin();
        let mut line = String::new();

        loop {
            print_prompt(&self.state)?;

            line.clear();

            if stdin.read_line(&mut line)? == 0 {
                break;
            }

            let input = line.trim();

            if input.is_empty() {
                continue;
            }

            self.state.command_count += 1;
            self.state.history.push(input.to_string());

            let keep_running = if input.starts_with(':') {
                commands::handle_command(
                    input,
                    &mut self.terminal,
                    &mut self.state,
                    self.renderer.as_ref(),
                )?
            } else {
                self.terminal.feed(input);
                self.terminal.feed("\n");
                self.renderer.render(&self.terminal);
                true
            };

            if !keep_running {
                break;
            }
        }

        println!();
        println!("Session terminated.");
        println!("Commands entered: {}", self.state.command_count);
        println!("Goodbye!");

        Ok(())
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
