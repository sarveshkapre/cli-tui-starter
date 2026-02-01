use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "cli-tui-starter",
    version,
    about = "Minimal TUI starter with themes and accessibility"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launch the interactive TUI demo.
    Demo(DemoArgs),
    /// List available themes.
    Themes,
    /// Print key bindings.
    Keys,
}

#[derive(Args, Debug, Clone)]
pub struct DemoArgs {
    /// Theme to use (by name). Defaults to "aurora".
    #[arg(long, value_enum, default_value_t = ThemeName::Aurora)]
    pub theme: ThemeName,
    /// Disable color output for maximum compatibility.
    #[arg(long, default_value_t = false)]
    pub no_color: bool,
    /// Use a high-contrast palette for better visibility.
    #[arg(long, default_value_t = false)]
    pub high_contrast: bool,
    /// Reduce motion (slower refresh, no animation).
    #[arg(long, default_value_t = false)]
    pub reduced_motion: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    Aurora,
    Mono,
    Solar,
}

impl ThemeName {
    pub fn as_str(self) -> &'static str {
        match self {
            ThemeName::Aurora => "aurora",
            ThemeName::Mono => "mono",
            ThemeName::Solar => "solar",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_name_strings_are_stable() {
        assert_eq!(ThemeName::Aurora.as_str(), "aurora");
        assert_eq!(ThemeName::Mono.as_str(), "mono");
        assert_eq!(ThemeName::Solar.as_str(), "solar");
    }
}
