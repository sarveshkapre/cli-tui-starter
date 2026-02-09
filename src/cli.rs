use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

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
    Keys(KeysArgs),
    /// Manage config files.
    Config(ConfigArgs),
}

#[derive(Args, Debug, Clone)]
pub struct DemoArgs {
    /// Theme to use (by name).
    #[arg(long, value_enum)]
    pub theme: Option<ThemeName>,
    /// Disable color output for maximum compatibility.
    #[arg(
        long,
        action = ArgAction::SetTrue,
        conflicts_with = "color",
        default_value_t = false
    )]
    pub no_color: bool,
    /// Force-enable color output and override config/environment no-color defaults.
    #[arg(long, action = ArgAction::SetTrue, default_value_t = false)]
    pub color: bool,
    /// Use a high-contrast palette for better visibility.
    #[arg(
        long,
        action = ArgAction::SetTrue,
        conflicts_with = "normal_contrast",
        default_value_t = false
    )]
    pub high_contrast: bool,
    /// Disable high-contrast mode.
    #[arg(long, action = ArgAction::SetTrue, default_value_t = false)]
    pub normal_contrast: bool,
    /// Reduce motion (slower refresh, no animation).
    #[arg(
        long,
        action = ArgAction::SetTrue,
        conflicts_with = "motion",
        default_value_t = false
    )]
    pub reduced_motion: bool,
    /// Force-enable motion and override config reduced-motion defaults.
    #[arg(long, action = ArgAction::SetTrue, default_value_t = false)]
    pub motion: bool,
    /// Optional path to config file (TOML).
    #[arg(long)]
    pub config: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct KeysArgs {
    /// Optional path to config file (TOML). When omitted, the default config path is used if it exists.
    #[arg(long)]
    pub config: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    /// Write a commented starter config file to the default config path.
    Init(ConfigInitArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ConfigInitArgs {
    /// Overwrite the config file if it already exists.
    #[arg(long, action = ArgAction::SetTrue, default_value_t = false)]
    pub force: bool,
    /// Print the starter config to stdout instead of writing a file.
    #[arg(long, action = ArgAction::SetTrue, default_value_t = false)]
    pub stdout: bool,
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

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "aurora" => Some(ThemeName::Aurora),
            "mono" => Some(ThemeName::Mono),
            "solar" => Some(ThemeName::Solar),
            _ => None,
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

    #[test]
    fn theme_name_parsing_is_case_insensitive() {
        assert_eq!(ThemeName::parse("AURORA"), Some(ThemeName::Aurora));
        assert_eq!(ThemeName::parse("mono"), Some(ThemeName::Mono));
        assert_eq!(ThemeName::parse(" Solar "), Some(ThemeName::Solar));
        assert_eq!(ThemeName::parse("unknown"), None);
    }
}
