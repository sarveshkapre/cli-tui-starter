mod app;
mod cli;
mod config;
mod keys;
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
        Commands::Keys(args) => print_keys(args),
    }
}

fn run_demo(args: cli::DemoArgs) -> Result<()> {
    let resolved = config::resolve_demo_runtime(&args)?;

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
        resolved.settings.theme,
        resolved.settings.no_color,
        resolved.settings.high_contrast,
        resolved.settings.reduced_motion,
        resolved.keys,
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

fn print_keys(args: cli::KeysArgs) -> Result<()> {
    let keymap = config::resolve_key_bindings(args.config.as_deref())?;
    let mut out = String::new();
    out.push_str("Key bindings:\n");
    out.push_str(&format!(
        "- {}: cycle theme\n",
        keys::key_list_display(&keymap.cycle_theme)
    ));
    out.push_str(&format!(
        "- {}: toggle high contrast\n",
        keys::key_list_display(&keymap.toggle_high_contrast)
    ));
    out.push_str(&format!(
        "- {}: toggle color\n",
        keys::key_list_display(&keymap.toggle_color)
    ));
    out.push_str(&format!(
        "- {}: toggle reduced motion\n",
        keys::key_list_display(&keymap.toggle_reduced_motion)
    ));
    out.push_str(&format!(
        "- {}: toggle help\n",
        keys::key_list_display(&keymap.toggle_help)
    ));
    out.push_str(&format!("- {}: quit\n", keymap.quit_label()));
    print!("{}", out);
    Ok(())
}
