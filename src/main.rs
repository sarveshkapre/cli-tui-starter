mod app;
mod cli;
mod config;
mod terminal;
mod theme;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use cli::{Cli, Commands};
use crossterm::event::{self, Event};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::IsTerminal;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Demo(args) => run_demo(args),
        Commands::Themes => {
            print_themes();
            Ok(())
        }
        Commands::Keys => {
            print_keys();
            Ok(())
        }
    }
}

fn run_demo(args: cli::DemoArgs) -> Result<()> {
    let settings = config::resolve_demo_settings(&args)?;

    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        anyhow::bail!(
            "`demo` is an interactive TUI and requires a real terminal (TTY). Try running it \
             directly in a terminal, or use `cli-tui-starter themes` / `cli-tui-starter keys`."
        );
    }

    let mut guard = terminal::TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(guard.stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(
        settings.theme,
        settings.no_color,
        settings.high_contrast,
        settings.reduced_motion,
    );
    let mut last_tick = Instant::now();
    let tick_rate = if app.reduced_motion {
        Duration::from_millis(500)
    } else {
        Duration::from_millis(200)
    };

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    terminal.show_cursor()?;
    Ok(())
}

fn print_themes() {
    let mut out = String::new();
    out.push_str("Available themes:\n");
    for theme in theme::themes() {
        out.push_str(&format!("- {}: {}\n", theme.name, theme.description));
    }
    print!("{}", out);
}

fn print_keys() {
    let mut out = String::new();
    out.push_str("Key bindings:\n");
    out.push_str("- t: cycle theme\n");
    out.push_str("- h: toggle high contrast\n");
    out.push_str("- c: toggle color\n");
    out.push_str("- r: toggle reduced motion\n");
    out.push_str("- ?: toggle help\n");
    out.push_str("- q/esc: quit\n");
    print!("{}", out);
}
