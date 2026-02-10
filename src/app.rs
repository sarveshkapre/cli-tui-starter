use crate::cli::ThemeName;
use crate::keys::KeyBindings;
use crate::theme::{themes, Theme};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoPanel {
    Overview,
    List,
}

impl DemoPanel {
    fn next(self) -> Self {
        match self {
            DemoPanel::Overview => DemoPanel::List,
            DemoPanel::List => DemoPanel::Overview,
        }
    }

    fn prev(self) -> Self {
        match self {
            DemoPanel::Overview => DemoPanel::List,
            DemoPanel::List => DemoPanel::Overview,
        }
    }

    pub fn index(self) -> usize {
        match self {
            DemoPanel::Overview => 0,
            DemoPanel::List => 1,
        }
    }
}

pub struct App {
    themes: Vec<Theme>,
    theme_index: usize,
    pub keymap: KeyBindings,
    pub no_color: bool,
    pub high_contrast: bool,
    pub reduced_motion: bool,
    panel: DemoPanel,
    list_selected: usize,
    pub show_help: bool,
    pub should_quit: bool,
    spinner_index: usize,
}

impl App {
    pub fn new(
        theme: ThemeName,
        no_color: bool,
        high_contrast: bool,
        reduced_motion: bool,
        keymap: KeyBindings,
        panel: DemoPanel,
    ) -> Self {
        let list = themes();
        let theme_index = list
            .iter()
            .position(|item| item.name == theme.as_str())
            .unwrap_or(0);

        Self {
            themes: list,
            theme_index,
            keymap,
            no_color,
            high_contrast,
            reduced_motion,
            panel,
            list_selected: 0,
            show_help: false,
            should_quit: false,
            spinner_index: 0,
        }
    }

    pub fn panel(&self) -> DemoPanel {
        self.panel
    }

    pub fn list_selected(&self) -> usize {
        self.list_selected
    }

    pub fn list_len(&self) -> usize {
        list_demo_len()
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
        if key.code == KeyCode::Esc
            || (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL)
            || KeyBindings::matches_any(&self.keymap.quit, key)
        {
            self.should_quit = true;
            return;
        }

        if KeyBindings::matches_any(&self.keymap.cycle_theme, key) {
            self.theme_index = (self.theme_index + 1) % self.themes.len();
            return;
        }
        if KeyBindings::matches_any(&self.keymap.next_panel, key) {
            self.panel = self.panel.next();
            return;
        }
        if KeyBindings::matches_any(&self.keymap.prev_panel, key) {
            self.panel = self.panel.prev();
            return;
        }
        if self.panel == DemoPanel::List {
            if KeyBindings::matches_any(&self.keymap.list_up, key) {
                self.list_selected = self.list_selected.saturating_sub(1);
                return;
            }
            if KeyBindings::matches_any(&self.keymap.list_down, key) {
                let max = list_demo_len().saturating_sub(1);
                self.list_selected = (self.list_selected + 1).min(max);
                return;
            }
        }
        if KeyBindings::matches_any(&self.keymap.toggle_high_contrast, key) {
            self.high_contrast = !self.high_contrast;
            return;
        }
        if KeyBindings::matches_any(&self.keymap.toggle_color, key) {
            self.no_color = !self.no_color;
            return;
        }
        if KeyBindings::matches_any(&self.keymap.toggle_reduced_motion, key) {
            self.reduced_motion = !self.reduced_motion;
            return;
        }
        if KeyBindings::matches_any(&self.keymap.toggle_help, key) {
            self.show_help = !self.show_help;
        }
    }
}

fn list_demo_len() -> usize {
    40
}
