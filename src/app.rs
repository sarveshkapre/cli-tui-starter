use crate::cli::DemoArgs;
use crate::theme::{themes, Theme};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct App {
    themes: Vec<Theme>,
    theme_index: usize,
    pub no_color: bool,
    pub high_contrast: bool,
    pub reduced_motion: bool,
    pub show_help: bool,
    pub should_quit: bool,
    spinner_index: usize,
}

impl App {
    pub fn new(args: DemoArgs) -> Self {
        let list = themes();
        let theme_index = list
            .iter()
            .position(|theme| theme.name == args.theme.as_str())
            .unwrap_or(0);

        Self {
            themes: list,
            theme_index,
            no_color: args.no_color,
            high_contrast: args.high_contrast,
            reduced_motion: args.reduced_motion,
            show_help: false,
            should_quit: false,
            spinner_index: 0,
        }
    }

    pub fn current_theme(&self) -> Theme {
        self.themes[self.theme_index]
            .clone()
            .with_accessibility(self.high_contrast, self.no_color)
    }

    pub fn current_theme_name(&self) -> &str {
        self.themes[self.theme_index].name
    }

    pub fn current_theme_description(&self) -> &str {
        self.themes[self.theme_index].description
    }

    pub fn spinner_frame(&self) -> &str {
        if self.no_color || self.reduced_motion {
            "•"
        } else {
            const FRAMES: [&str; 4] = ["-", "\\", "|", "/"];
            FRAMES[self.spinner_index % FRAMES.len()]
        }
    }

    pub fn tick(&mut self) {
        if !self.no_color && !self.reduced_motion {
            self.spinner_index = self.spinner_index.wrapping_add(1);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }
            | KeyEvent {
                code: KeyCode::Esc, ..
            } => {
                self.should_quit = true;
            }
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_quit = true;
            }
            KeyEvent {
                code: KeyCode::Char('t'),
                ..
            } => {
                self.theme_index = (self.theme_index + 1) % self.themes.len();
            }
            KeyEvent {
                code: KeyCode::Char('h'),
                ..
            } => {
                self.high_contrast = !self.high_contrast;
            }
            KeyEvent {
                code: KeyCode::Char('c'),
                ..
            } => {
                self.no_color = !self.no_color;
            }
            KeyEvent {
                code: KeyCode::Char('r'),
                ..
            } => {
                self.reduced_motion = !self.reduced_motion;
            }
            KeyEvent {
                code: KeyCode::Char('?'),
                ..
            } => {
                self.show_help = !self.show_help;
            }
            _ => {}
        }
    }
}
