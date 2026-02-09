use crate::cli::{DemoArgs, ThemeName};
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

pub fn resolve_demo_settings(args: &DemoArgs) -> Result<DemoSettings> {
    let defaults = load_demo_defaults(args.config.as_deref())?;
    Ok(resolve_with_sources(
        args,
        &defaults,
        env_disables_color_current(),
    ))
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    #[serde(default)]
    demo: DemoDefaultsRaw,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct DemoDefaultsRaw {
    theme: Option<String>,
    no_color: Option<bool>,
    high_contrast: Option<bool>,
    reduced_motion: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DemoDefaults {
    theme: Option<ThemeName>,
    no_color: Option<bool>,
    high_contrast: Option<bool>,
    reduced_motion: Option<bool>,
}

fn load_demo_defaults(path_override: Option<&Path>) -> Result<DemoDefaults> {
    let config_path = match path_override {
        Some(path) => {
            if !path.exists() {
                bail!("config file not found: {}", path.display());
            }
            path.to_path_buf()
        }
        None => match default_config_path() {
            Some(path) if path.exists() => path,
            _ => return Ok(DemoDefaults::default()),
        },
    };

    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config file: {}", config_path.display()))?;
    parse_demo_defaults(&contents, &config_path)
}

fn parse_demo_defaults(contents: &str, source: &Path) -> Result<DemoDefaults> {
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

    Ok(DemoDefaults {
        theme,
        no_color: raw.demo.no_color,
        high_contrast: raw.demo.high_contrast,
        reduced_motion: raw.demo.reduced_motion,
    })
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

fn default_config_path() -> Option<PathBuf> {
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
    fn parse_demo_defaults_reads_demo_section() {
        let path = Path::new("/tmp/config.toml");
        let parsed = parse_demo_defaults(
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

        assert_eq!(
            parsed,
            DemoDefaults {
                theme: Some(ThemeName::Solar),
                no_color: Some(true),
                high_contrast: Some(false),
                reduced_motion: Some(true),
            }
        );
    }

    #[test]
    fn parse_demo_defaults_rejects_unknown_theme() {
        let path = Path::new("/tmp/config.toml");
        let error = parse_demo_defaults(
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
