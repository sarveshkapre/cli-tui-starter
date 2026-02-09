use anyhow::{anyhow, bail, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeySpec {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeySpec {
    pub fn matches(self, event: KeyEvent) -> bool {
        // Crossterm can set extra modifier bits; we want exact matches for now.
        event.code == self.code && event.modifiers == self.modifiers
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBindings {
    pub cycle_theme: Vec<KeySpec>,
    pub toggle_high_contrast: Vec<KeySpec>,
    pub toggle_color: Vec<KeySpec>,
    pub toggle_reduced_motion: Vec<KeySpec>,
    pub toggle_help: Vec<KeySpec>,
    /// Additional quit keys. `Esc` and `Ctrl+C` are treated as emergency quit regardless.
    pub quit: Vec<KeySpec>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            cycle_theme: vec![parse_key_spec("t").expect("default key spec")],
            toggle_high_contrast: vec![parse_key_spec("h").expect("default key spec")],
            toggle_color: vec![parse_key_spec("c").expect("default key spec")],
            toggle_reduced_motion: vec![parse_key_spec("r").expect("default key spec")],
            toggle_help: vec![parse_key_spec("?").expect("default key spec")],
            quit: vec![parse_key_spec("q").expect("default key spec")],
        }
    }
}

impl KeyBindings {
    pub fn validate(&self) -> Result<()> {
        fn ensure_non_empty(name: &str, keys: &[KeySpec]) -> Result<()> {
            if keys.is_empty() {
                bail!("key binding '{}' must not be empty", name);
            }
            Ok(())
        }

        ensure_non_empty("cycle_theme", &self.cycle_theme)?;
        ensure_non_empty("toggle_high_contrast", &self.toggle_high_contrast)?;
        ensure_non_empty("toggle_color", &self.toggle_color)?;
        ensure_non_empty("toggle_reduced_motion", &self.toggle_reduced_motion)?;
        ensure_non_empty("toggle_help", &self.toggle_help)?;
        ensure_non_empty("quit", &self.quit)?;

        // `Esc` and `Ctrl+C` are reserved as emergency quit. They should not appear in other bindings
        // to avoid confusing UX in the help panel.
        let reserved = [parse_key_spec("esc")?, parse_key_spec("ctrl+c")?];

        let mut seen = HashSet::<KeySpec>::new();
        for (name, keys) in [
            ("cycle_theme", &self.cycle_theme),
            ("toggle_high_contrast", &self.toggle_high_contrast),
            ("toggle_color", &self.toggle_color),
            ("toggle_reduced_motion", &self.toggle_reduced_motion),
            ("toggle_help", &self.toggle_help),
            ("quit", &self.quit),
        ] {
            for key in keys.iter().copied() {
                if reserved.contains(&key) && name != "quit" {
                    bail!(
                        "key '{}' is reserved for quitting and cannot be used for '{}'",
                        key_spec_display(key),
                        name
                    );
                }
                if !seen.insert(key) {
                    bail!(
                        "duplicate key binding '{}' used for multiple actions",
                        key_spec_display(key)
                    );
                }
            }
        }

        Ok(())
    }

    pub fn matches_any(keys: &[KeySpec], event: KeyEvent) -> bool {
        keys.iter().copied().any(|k| k.matches(event))
    }

    pub fn quit_label(&self) -> String {
        // Always advertise `esc` as a safe exit path even if it's not set in config.
        let mut out = Vec::<String>::new();
        let mut seen = HashSet::<String>::new();

        for &k in &self.quit {
            let label = key_spec_display(k);
            if seen.insert(label.clone()) {
                out.push(label);
            }
        }

        if seen.insert("esc".to_string()) {
            out.push("esc".to_string());
        }

        out.join("/")
    }
}

pub fn parse_key_spec(value: &str) -> Result<KeySpec> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        bail!("empty key spec");
    }

    let mut modifiers = KeyModifiers::empty();
    let parts: Vec<&str> = trimmed.split('+').map(|p| p.trim()).collect();
    let key_part = if parts.len() > 1 {
        let key = parts.last().copied().unwrap_or("");
        for m in &parts[..parts.len() - 1] {
            match m.to_ascii_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= KeyModifiers::CONTROL,
                "alt" => modifiers |= KeyModifiers::ALT,
                "shift" => modifiers |= KeyModifiers::SHIFT,
                other => bail!("unsupported modifier '{}'", other),
            }
        }
        key
    } else {
        trimmed
    };

    let key_lower = key_part.to_ascii_lowercase();
    let code = match key_lower.as_str() {
        "esc" | "escape" => KeyCode::Esc,
        "enter" | "return" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backtab" => KeyCode::BackTab,
        "space" => KeyCode::Char(' '),
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        _ => {
            // Accept a single Unicode scalar as a char key (e.g. "t" or "?").
            let mut chars = key_part.chars();
            let ch = chars.next().ok_or_else(|| anyhow!("empty key spec"))?;
            if chars.next().is_some() {
                bail!("unsupported multi-character key spec '{}'", key_part);
            }
            KeyCode::Char(ch)
        }
    };

    Ok(KeySpec { code, modifiers })
}

pub fn key_spec_display(spec: KeySpec) -> String {
    let mut out = String::new();
    if spec.modifiers.contains(KeyModifiers::CONTROL) {
        out.push_str("ctrl+");
    }
    if spec.modifiers.contains(KeyModifiers::ALT) {
        out.push_str("alt+");
    }
    if spec.modifiers.contains(KeyModifiers::SHIFT) {
        out.push_str("shift+");
    }

    match spec.code {
        KeyCode::Char(' ') => out.push_str("space"),
        KeyCode::Char(c) => out.push(c),
        KeyCode::Esc => out.push_str("esc"),
        KeyCode::Enter => out.push_str("enter"),
        KeyCode::Tab => out.push_str("tab"),
        KeyCode::BackTab => out.push_str("backtab"),
        KeyCode::Up => out.push_str("up"),
        KeyCode::Down => out.push_str("down"),
        KeyCode::Left => out.push_str("left"),
        KeyCode::Right => out.push_str("right"),
        _ => out.push_str("unknown"),
    }

    out
}

pub fn key_list_display(keys: &[KeySpec]) -> String {
    let mut out = Vec::<String>::new();
    let mut seen = HashSet::<String>::new();

    for &k in keys {
        let label = key_spec_display(k);
        if seen.insert(label.clone()) {
            out.push(label);
        }
    }

    out.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_chars() {
        assert_eq!(
            parse_key_spec("t").unwrap(),
            KeySpec {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::empty()
            }
        );
        assert_eq!(
            parse_key_spec("?").unwrap(),
            KeySpec {
                code: KeyCode::Char('?'),
                modifiers: KeyModifiers::empty()
            }
        );
    }

    #[test]
    fn parse_special_keys_and_modifiers() {
        assert_eq!(
            parse_key_spec("esc").unwrap(),
            KeySpec {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::empty()
            }
        );
        assert_eq!(
            parse_key_spec("ctrl+c").unwrap(),
            KeySpec {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL
            }
        );
    }

    #[test]
    fn default_keymap_is_valid() {
        KeyBindings::default().validate().unwrap();
    }
}
