use crate::render::Renderer;
use retro_terminal::Terminal;

pub fn boot_demo(profile: &str, terminal: &mut Terminal, renderer: &dyn Renderer) {
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

    renderer.render(terminal);
}
