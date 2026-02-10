mod app;
mod cli;
mod config;
mod keys;
mod terminal;
mod theme;
mod ui;

use anyhow::Result;
use app::App;
use app::DemoPanel;
use clap::Parser;
use cli::{Cli, Commands};
use crossterm::event::{self, Event};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use serde::Serialize;
use std::io::IsTerminal;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Demo(args) => run_demo(args),
        Commands::Themes(args) => print_themes(args),
        Commands::Keys(args) => print_keys(args),
        Commands::Config(args) => run_config(args),
    }
}

fn run_demo(args: cli::DemoArgs) -> Result<()> {
    let resolved = config::resolve_demo_runtime(&args)?;

    if args.no_tty {
        let width = args.width.unwrap_or(80).clamp(20, 240);
        let height = args.height.unwrap_or(24).clamp(10, 120);

        let app = App::new(
            resolved.settings.theme,
            resolved.settings.no_color,
            resolved.settings.high_contrast,
            resolved.settings.reduced_motion,
            resolved.keys,
            DemoPanel::Overview,
        );
        print!(
            "{}",
            ui::render_static_preview(&app, width, height, resolved.settings.ascii)?
        );
        return Ok(());
    }

    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        anyhow::bail!(
            "`demo` is an interactive TUI and requires a real terminal (TTY). Try running it \
             directly in a terminal, use `cli-tui-starter demo --no-tty` for a static preview, \
             or run `cli-tui-starter themes` / `cli-tui-starter keys`."
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
        DemoPanel::Overview,
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

fn run_config(args: cli::ConfigArgs) -> Result<()> {
    match args.command {
        cli::ConfigCommands::Init(init) => config_init(init),
        cli::ConfigCommands::Validate(validate) => config_validate(validate),
    }
}

fn config_init(args: cli::ConfigInitArgs) -> Result<()> {
    if args.stdout {
        print!("{}", config::starter_config_toml());
        return Ok(());
    }

    let path = config::default_config_path()
        .ok_or_else(|| anyhow::anyhow!("cannot determine default config path (HOME not set)"))?;

    if path.exists() && !args.force {
        anyhow::bail!(
            "config already exists at {} (use `cli-tui-starter config init --force` to overwrite)",
            path.display()
        );
    }

    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("invalid config path: {}", path.display()))?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(&path, config::starter_config_toml())?;

    println!("Wrote config: {}", path.display());
    Ok(())
}

fn config_validate(args: cli::ConfigValidateArgs) -> Result<()> {
    let path = match args.config {
        Some(path) => path,
        None => {
            let path = config::default_config_path().ok_or_else(|| {
                anyhow::anyhow!("cannot determine default config path (HOME not set)")
            })?;
            if !path.exists() {
                anyhow::bail!(
                    "config file not found at {} (try `cli-tui-starter config init`)",
                    path.display()
                );
            }
            path
        }
    };

    config::validate_config_file(&path)?;
    match args.format {
        cli::OutputFormat::Text => {
            println!("Config OK: {}", path.display());
        }
        cli::OutputFormat::Json => {
            #[derive(Serialize)]
            struct ConfigValidateJson {
                ok: bool,
                path: String,
            }
            let payload = ConfigValidateJson {
                ok: true,
                path: path.display().to_string(),
            };
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct ThemesJson<'a> {
    themes: Vec<ThemeInfo<'a>>,
}

#[derive(Serialize)]
struct ThemeInfo<'a> {
    name: &'a str,
    description: &'a str,
}

fn print_themes(args: cli::ThemesArgs) -> Result<()> {
    match args.format {
        cli::OutputFormat::Text => {
            let mut out = String::new();
            out.push_str("Available themes:\n");
            for theme in theme::themes() {
                out.push_str(&format!("- {}: {}\n", theme.name, theme.description));
            }
            print!("{}", out);
            Ok(())
        }
        cli::OutputFormat::Json => {
            let list = theme::themes();
            let payload = ThemesJson {
                themes: list
                    .iter()
                    .map(|t| ThemeInfo {
                        name: t.name,
                        description: t.description,
                    })
                    .collect(),
            };
            println!("{}", serde_json::to_string_pretty(&payload)?);
            Ok(())
        }
    }
}

fn print_keys(args: cli::KeysArgs) -> Result<()> {
    let keymap = config::resolve_key_bindings(args.config.as_deref())?;

    match args.format {
        cli::OutputFormat::Text => {
            let mut out = String::new();
            out.push_str("Key bindings:\n");
            out.push_str(&format!(
                "- {}: cycle theme\n",
                keys::key_list_display(&keymap.cycle_theme)
            ));
            out.push_str(&format!(
                "- {}: next panel\n",
                keys::key_list_display(&keymap.next_panel)
            ));
            out.push_str(&format!(
                "- {}: previous panel\n",
                keys::key_list_display(&keymap.prev_panel)
            ));
            out.push_str(&format!(
                "- {}: list up\n",
                keys::key_list_display(&keymap.list_up)
            ));
            out.push_str(&format!(
                "- {}: list down\n",
                keys::key_list_display(&keymap.list_down)
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
        cli::OutputFormat::Json => {
            #[derive(Serialize)]
            struct KeysJson {
                cycle_theme: Vec<String>,
                next_panel: Vec<String>,
                prev_panel: Vec<String>,
                list_up: Vec<String>,
                list_down: Vec<String>,
                toggle_high_contrast: Vec<String>,
                toggle_color: Vec<String>,
                toggle_reduced_motion: Vec<String>,
                toggle_help: Vec<String>,
                quit: Vec<String>,
            }

            fn labels(list: &[keys::KeySpec]) -> Vec<String> {
                use std::collections::HashSet;
                let mut out = Vec::<String>::new();
                let mut seen = HashSet::<String>::new();
                for &k in list {
                    let label = keys::key_spec_display(k);
                    if seen.insert(label.clone()) {
                        out.push(label);
                    }
                }
                out
            }

            let payload = KeysJson {
                cycle_theme: labels(&keymap.cycle_theme),
                next_panel: labels(&keymap.next_panel),
                prev_panel: labels(&keymap.prev_panel),
                list_up: labels(&keymap.list_up),
                list_down: labels(&keymap.list_down),
                toggle_high_contrast: labels(&keymap.toggle_high_contrast),
                toggle_color: labels(&keymap.toggle_color),
                toggle_reduced_motion: labels(&keymap.toggle_reduced_motion),
                toggle_help: labels(&keymap.toggle_help),
                quit: keymap.quit_labels(),
            };
            println!("{}", serde_json::to_string_pretty(&payload)?);
            Ok(())
        }
    }
}
