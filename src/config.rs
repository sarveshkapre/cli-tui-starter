use crate::cli::{DemoArgs, ThemeName};
use crate::keys::{parse_key_spec, KeyBindings, KeySpec};
use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DemoSettings {
    pub theme: ThemeName,
    pub no_color: bool,
    pub high_contrast: bool,
    pub reduced_motion: bool,
}

pub struct DemoRuntime {
    pub settings: DemoSettings,
    pub keys: KeyBindings,
}

pub fn resolve_demo_runtime(args: &DemoArgs) -> Result<DemoRuntime> {
    let loaded = load_config_bundle(args.config.as_deref())?;
    let settings = resolve_with_sources(args, &loaded.demo, env_disables_color_current());
    Ok(DemoRuntime {
        settings,
        keys: loaded.keys,
    })
}

pub fn resolve_key_bindings(path_override: Option<&Path>) -> Result<KeyBindings> {
    Ok(load_config_bundle(path_override)?.keys)
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    #[serde(default)]
    demo: DemoDefaultsRaw,
    #[serde(default)]
    keys: KeysOverridesRaw,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct DemoDefaultsRaw {
    theme: Option<String>,
    no_color: Option<bool>,
    high_contrast: Option<bool>,
    reduced_motion: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum OneOrManyStrings {
    One(String),
    Many(Vec<String>),
}

impl OneOrManyStrings {
    fn into_vec(self) -> Vec<String> {
        match self {
            OneOrManyStrings::One(v) => vec![v],
            OneOrManyStrings::Many(v) => v,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct KeysOverridesRaw {
    cycle_theme: Option<OneOrManyStrings>,
    toggle_high_contrast: Option<OneOrManyStrings>,
    toggle_color: Option<OneOrManyStrings>,
    toggle_reduced_motion: Option<OneOrManyStrings>,
    toggle_help: Option<OneOrManyStrings>,
    quit: Option<OneOrManyStrings>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DemoDefaults {
    theme: Option<ThemeName>,
    no_color: Option<bool>,
    high_contrast: Option<bool>,
    reduced_motion: Option<bool>,
}

#[derive(Debug, Clone)]
struct LoadedConfigBundle {
    demo: DemoDefaults,
    keys: KeyBindings,
}

fn load_config_bundle(path_override: Option<&Path>) -> Result<LoadedConfigBundle> {
    let config_path = match path_override {
        Some(path) => {
            if !path.exists() {
                bail!("config file not found: {}", path.display());
            }
            Some(path.to_path_buf())
        }
        None => match default_config_path() {
            Some(path) if path.exists() => Some(path),
            _ => None,
        },
    };

    let Some(config_path) = config_path else {
        return Ok(LoadedConfigBundle {
            demo: DemoDefaults::default(),
            keys: KeyBindings::default(),
        });
    };

    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config file: {}", config_path.display()))?;
    parse_config_bundle(&contents, &config_path)
}

fn parse_config_bundle(contents: &str, source: &Path) -> Result<LoadedConfigBundle> {
    let raw: FileConfig = toml::from_str(contents)
        .with_context(|| format!("invalid config TOML in {}", source.display()))?;

    let theme = match raw.demo.theme {
        Some(name) => Some(ThemeName::parse(&name).ok_or_else(|| {
            anyhow!(
                "invalid theme '{}' in {}. valid themes: aurora, mono, solar",
                name,
                source.display()
            )
        })?),
        None => None,
    };

    let demo = DemoDefaults {
        theme,
        no_color: raw.demo.no_color,
        high_contrast: raw.demo.high_contrast,
        reduced_motion: raw.demo.reduced_motion,
    };

    let keys = apply_keys_overrides(KeyBindings::default(), raw.keys, source)?;

    Ok(LoadedConfigBundle { demo, keys })
}

fn apply_keys_overrides(
    mut keymap: KeyBindings,
    overrides: KeysOverridesRaw,
    source: &Path,
) -> Result<KeyBindings> {
    fn parse_list(values: OneOrManyStrings, source: &Path, name: &str) -> Result<Vec<KeySpec>> {
        let raw = values.into_vec();
        if raw.is_empty() {
            bail!(
                "key binding '{}' in {} must not be empty",
                name,
                source.display()
            );
        }
        raw.iter()
            .map(|s| {
                parse_key_spec(s).with_context(|| {
                    format!(
                        "invalid key spec '{}' for '{}' in {}",
                        s,
                        name,
                        source.display()
                    )
                })
            })
            .collect()
    }

    if let Some(v) = overrides.cycle_theme {
        keymap.cycle_theme = parse_list(v, source, "cycle_theme")?;
    }
    if let Some(v) = overrides.toggle_high_contrast {
        keymap.toggle_high_contrast = parse_list(v, source, "toggle_high_contrast")?;
    }
    if let Some(v) = overrides.toggle_color {
        keymap.toggle_color = parse_list(v, source, "toggle_color")?;
    }
    if let Some(v) = overrides.toggle_reduced_motion {
        keymap.toggle_reduced_motion = parse_list(v, source, "toggle_reduced_motion")?;
    }
    if let Some(v) = overrides.toggle_help {
        keymap.toggle_help = parse_list(v, source, "toggle_help")?;
    }
    if let Some(v) = overrides.quit {
        keymap.quit = parse_list(v, source, "quit")?;
    }

    keymap
        .validate()
        .with_context(|| format!("invalid key bindings configuration in {}", source.display()))?;
    Ok(keymap)
}

fn resolve_with_sources(
    args: &DemoArgs,
    defaults: &DemoDefaults,
    env_no_color: bool,
) -> DemoSettings {
    let theme = args.theme.or(defaults.theme).unwrap_or(ThemeName::Aurora);

    let no_color = if args.no_color {
        true
    } else if args.color {
        false
    } else if let Some(value) = defaults.no_color {
        value
    } else {
        env_no_color
    };

    let high_contrast = if args.high_contrast {
        true
    } else if args.normal_contrast {
        false
    } else {
        defaults.high_contrast.unwrap_or(false)
    };

    let reduced_motion = if args.reduced_motion {
        true
    } else if args.motion {
        false
    } else {
        defaults.reduced_motion.unwrap_or(false)
    };

    DemoSettings {
        theme,
        no_color,
        high_contrast,
        reduced_motion,
    }
}

pub fn default_config_path() -> Option<PathBuf> {
    if let Some(path) = env::var_os("XDG_CONFIG_HOME") {
        return Some(
            PathBuf::from(path)
                .join("cli-tui-starter")
                .join("config.toml"),
        );
    }

    env::var_os("HOME").map(PathBuf::from).map(|home| {
        home.join(".config")
            .join("cli-tui-starter")
            .join("config.toml")
    })
}

pub fn starter_config_toml() -> &'static str {
    // Keep this ASCII-only, and keep defaults aligned with `Default` impls and README schema.
    r#"# cli-tui-starter config
#
# Precedence:
# 1) CLI flags
# 2) Config file values
# 3) Built-in defaults and NO_COLOR/CLICOLOR=0/TERM=dumb

[demo]
# theme = "aurora"         # aurora | mono | solar
theme = "aurora"
no_color = false
high_contrast = false
reduced_motion = false

[keys]
cycle_theme = "t"
toggle_high_contrast = "h"
toggle_color = "c"
toggle_reduced_motion = "r"
toggle_help = "?"
quit = ["q", "esc"]        # `esc` and `ctrl+c` always quit even if not listed
"#
}

fn env_disables_color_current() -> bool {
    env_disables_color(
        env::var_os("NO_COLOR").as_deref(),
        env::var_os("CLICOLOR").as_deref(),
        env::var_os("TERM").as_deref(),
    )
}

fn env_disables_color(
    no_color: Option<&OsStr>,
    clicolor: Option<&OsStr>,
    term: Option<&OsStr>,
) -> bool {
    if no_color.is_some() {
        return true;
    }

    if matches!(term.and_then(|t| t.to_str()), Some("dumb")) {
        return true;
    }

    matches!(clicolor.and_then(|c| c.to_str()), Some("0"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_args() -> DemoArgs {
        DemoArgs {
            theme: None,
            no_color: false,
            color: false,
            high_contrast: false,
            normal_contrast: false,
            reduced_motion: false,
            motion: false,
            config: None,
        }
    }

    #[test]
    fn parse_config_bundle_reads_demo_section() {
        let path = Path::new("/tmp/config.toml");
        let parsed = parse_config_bundle(
            r#"
            [demo]
            theme = "solar"
            no_color = true
            high_contrast = false
            reduced_motion = true
            "#,
            path,
        )
        .expect("config should parse");

        assert_eq!(parsed.demo.theme, Some(ThemeName::Solar));
        assert_eq!(parsed.demo.no_color, Some(true));
        assert_eq!(parsed.demo.high_contrast, Some(false));
        assert_eq!(parsed.demo.reduced_motion, Some(true));
    }

    #[test]
    fn parse_config_bundle_rejects_unknown_theme() {
        let path = Path::new("/tmp/config.toml");
        let error = parse_config_bundle(
            r#"
            [demo]
            theme = "neon"
            "#,
            path,
        )
        .expect_err("invalid theme must fail");

        assert!(error
            .to_string()
            .contains("valid themes: aurora, mono, solar"));
    }

    #[test]
    fn parse_config_bundle_reads_keys_section() {
        let path = Path::new("/tmp/config.toml");
        let parsed = parse_config_bundle(
            r#"
            [keys]
            cycle_theme = "n"
            toggle_help = ["?", "g"]
            quit = "x"
            "#,
            path,
        )
        .expect("config should parse");

        assert!(parsed
            .keys
            .cycle_theme
            .iter()
            .any(|k| k.code == crossterm::event::KeyCode::Char('n')));
        assert!(parsed
            .keys
            .toggle_help
            .iter()
            .any(|k| k.code == crossterm::event::KeyCode::Char('?')));
        assert!(parsed
            .keys
            .toggle_help
            .iter()
            .any(|k| k.code == crossterm::event::KeyCode::Char('g')));
        assert!(parsed
            .keys
            .quit
            .iter()
            .any(|k| k.code == crossterm::event::KeyCode::Char('x')));
    }

    #[test]
    fn parse_config_bundle_rejects_duplicate_keys_across_actions() {
        let path = Path::new("/tmp/config.toml");
        let error = parse_config_bundle(
            r#"
            [keys]
            cycle_theme = "t"
            toggle_color = "t"
            "#,
            path,
        )
        .expect_err("duplicates must fail");

        let msg = format!("{:#}", error);
        assert!(msg.contains("duplicate key binding"), "msg was: {}", msg);
    }

    #[test]
    fn resolve_prefers_cli_over_config_and_env() {
        let mut args = default_args();
        args.theme = Some(ThemeName::Mono);
        args.color = true;
        args.normal_contrast = true;
        args.motion = true;

        let defaults = DemoDefaults {
            theme: Some(ThemeName::Solar),
            no_color: Some(true),
            high_contrast: Some(true),
            reduced_motion: Some(true),
        };

        let resolved = resolve_with_sources(&args, &defaults, true);
        assert_eq!(
            resolved,
            DemoSettings {
                theme: ThemeName::Mono,
                no_color: false,
                high_contrast: false,
                reduced_motion: false,
            }
        );
    }

    #[test]
    fn resolve_prefers_config_over_env_default() {
        let args = default_args();
        let defaults = DemoDefaults {
            theme: Some(ThemeName::Solar),
            no_color: Some(false),
            high_contrast: Some(true),
            reduced_motion: Some(true),
        };

        let resolved = resolve_with_sources(&args, &defaults, true);
        assert_eq!(resolved.theme, ThemeName::Solar);
        assert!(!resolved.no_color);
        assert!(resolved.high_contrast);
        assert!(resolved.reduced_motion);
    }

    #[test]
    fn env_disables_color_cases() {
        assert!(env_disables_color(
            Some(OsStr::new("")),
            Some(OsStr::new("1")),
            Some(OsStr::new("xterm-256color"))
        ));
        assert!(env_disables_color(
            None,
            Some(OsStr::new("0")),
            Some(OsStr::new("xterm-256color"))
        ));
        assert!(env_disables_color(None, None, Some(OsStr::new("dumb"))));
        assert!(!env_disables_color(
            None,
            Some(OsStr::new("1")),
            Some(OsStr::new("xterm-256color"))
        ));
    }
}
