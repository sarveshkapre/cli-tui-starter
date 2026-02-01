use crate::cli::ThemeName;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub description: &'static str,
    pub palette: ThemePalette,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub fg: Color,
    pub bg: Color,
    pub accent: Color,
    pub muted: Color,
    pub success: Color,
    pub danger: Color,
}

impl Theme {
    pub fn with_accessibility(mut self, high_contrast: bool, no_color: bool) -> Self {
        if no_color {
            self.palette = ThemePalette {
                fg: Color::Reset,
                bg: Color::Reset,
                accent: Color::Reset,
                muted: Color::Reset,
                success: Color::Reset,
                danger: Color::Reset,
            };
            return self;
        }

        if high_contrast {
            self.palette = ThemePalette {
                fg: Color::White,
                bg: Color::Black,
                accent: Color::Yellow,
                muted: Color::Gray,
                success: Color::Green,
                danger: Color::Red,
            };
        }

        self
    }
}

pub fn themes() -> Vec<Theme> {
    vec![
        Theme {
            name: ThemeName::Aurora.as_str(),
            description: "Cool blues with a calm accent",
            palette: ThemePalette {
                fg: Color::White,
                bg: Color::Black,
                accent: Color::LightBlue,
                muted: Color::Gray,
                success: Color::LightGreen,
                danger: Color::LightRed,
            },
        },
        Theme {
            name: ThemeName::Mono.as_str(),
            description: "Neutral monochrome for maximum focus",
            palette: ThemePalette {
                fg: Color::White,
                bg: Color::Black,
                accent: Color::Gray,
                muted: Color::DarkGray,
                success: Color::White,
                danger: Color::White,
            },
        },
        Theme {
            name: ThemeName::Solar.as_str(),
            description: "Warm highlights with soft contrast",
            palette: ThemePalette {
                fg: Color::White,
                bg: Color::Black,
                accent: Color::Yellow,
                muted: Color::Gray,
                success: Color::LightGreen,
                danger: Color::LightRed,
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themes_have_unique_names() {
        let list = themes();
        let mut names: Vec<&str> = list.iter().map(|theme| theme.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), list.len());
    }
}
