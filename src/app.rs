use crate::cli::ThemeName;
use crate::keys::KeyBindings;
use crate::theme::{themes, Theme};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

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
    pub mouse_enabled: bool,
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
        Self::new_with_mouse(
            theme,
            no_color,
            high_contrast,
            reduced_motion,
            keymap,
            false,
            panel,
        )
    }

    pub fn new_with_mouse(
        theme: ThemeName,
        no_color: bool,
        high_contrast: bool,
        reduced_motion: bool,
        keymap: KeyBindings,
        mouse_enabled: bool,
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
            mouse_enabled,
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
                self.list_move_up();
                return;
            }
            if KeyBindings::matches_any(&self.keymap.list_down, key) {
                self.list_move_down();
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

    pub fn handle_mouse(&mut self, mouse: MouseEvent, area: Rect) {
        if !self.mouse_enabled {
            return;
        }

        match mouse.kind {
            MouseEventKind::ScrollUp => self.list_move_up(),
            MouseEventKind::ScrollDown => self.list_move_down(),
            MouseEventKind::Down(MouseButton::Left) => {
                self.handle_left_click(mouse.column, mouse.row, area)
            }
            _ => {}
        }
    }

    fn handle_left_click(&mut self, column: u16, row: u16, area: Rect) {
        let Some(regions) = MouseRegions::for_area(area) else {
            return;
        };

        if row == regions.tabs_row
            && column >= regions.tabs_x
            && column < regions.tabs_x.saturating_add(regions.tabs_width)
        {
            let split = regions.tabs_x.saturating_add(regions.tabs_width / 2);
            self.panel = if column < split {
                DemoPanel::Overview
            } else {
                DemoPanel::List
            };
            return;
        }

        if self.panel != DemoPanel::List || !point_in_rect(column, row, regions.list_area) {
            return;
        }

        let total = list_demo_len();
        if total == 0 || regions.list_area.height == 0 {
            return;
        }
        let viewport = regions.list_area.height as usize;
        let start = list_viewport_start(self.list_selected, total, viewport);
        let offset = (row - regions.list_area.y) as usize;
        self.list_selected = (start + offset).min(total.saturating_sub(1));
    }

    fn list_move_up(&mut self) {
        if self.panel == DemoPanel::List {
            self.list_selected = self.list_selected.saturating_sub(1);
        }
    }

    fn list_move_down(&mut self) {
        if self.panel == DemoPanel::List {
            let max = list_demo_len().saturating_sub(1);
            self.list_selected = (self.list_selected + 1).min(max);
        }
    }
}

fn list_demo_len() -> usize {
    40
}

fn list_viewport_start(selected: usize, total: usize, viewport: usize) -> usize {
    if total == 0 || viewport == 0 {
        return 0;
    }

    let mut start = if selected >= viewport {
        selected + 1 - viewport
    } else {
        0
    };
    start = start.min(total.saturating_sub(viewport));
    start
}

fn point_in_rect(column: u16, row: u16, rect: Rect) -> bool {
    column >= rect.x
        && column < rect.x.saturating_add(rect.width)
        && row >= rect.y
        && row < rect.y.saturating_add(rect.height)
}

struct MouseRegions {
    tabs_row: u16,
    tabs_x: u16,
    tabs_width: u16,
    list_area: Rect,
}

impl MouseRegions {
    fn for_area(area: Rect) -> Option<Self> {
        if area.width <= 2 || area.height <= 2 {
            return None;
        }

        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);
        let body = root[1];
        if body.height == 0 {
            return None;
        }

        let showcase = if area.width < 90 {
            let stack = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),
                    Constraint::Min(0),
                    Constraint::Length(4),
                ])
                .split(body);
            stack[1]
        } else {
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
                .split(body);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(columns[1]);
            right[0]
        };

        if showcase.width <= 2 || showcase.height <= 2 {
            return None;
        }

        let inner = Rect {
            x: showcase.x.saturating_add(1),
            y: showcase.y.saturating_add(1),
            width: showcase.width.saturating_sub(2),
            height: showcase.height.saturating_sub(2),
        };
        if inner.width == 0 || inner.height == 0 {
            return None;
        }

        let showcase_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(inner);
        if showcase_layout[1].height == 0 {
            return None;
        }

        let list_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(showcase_layout[1]);

        Some(Self {
            tabs_row: showcase_layout[0].y,
            tabs_x: showcase_layout[0].x,
            tabs_width: showcase_layout[0].width,
            list_area: list_layout[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    fn mouse_event(kind: MouseEventKind, column: u16, row: u16) -> MouseEvent {
        MouseEvent {
            kind,
            column,
            row,
            modifiers: KeyModifiers::empty(),
        }
    }

    #[test]
    fn mouse_scroll_moves_list_when_enabled() {
        let mut app = App::new_with_mouse(
            ThemeName::Aurora,
            true,
            false,
            true,
            KeyBindings::default(),
            true,
            DemoPanel::List,
        );
        let area = Rect::new(0, 0, 120, 24);

        app.handle_mouse(mouse_event(MouseEventKind::ScrollDown, 10, 10), area);
        assert_eq!(app.list_selected(), 1);
        app.handle_mouse(mouse_event(MouseEventKind::ScrollUp, 10, 10), area);
        assert_eq!(app.list_selected(), 0);
    }

    #[test]
    fn mouse_scroll_is_ignored_when_disabled() {
        let mut app = App::new(
            ThemeName::Aurora,
            true,
            false,
            true,
            KeyBindings::default(),
            DemoPanel::List,
        );
        let area = Rect::new(0, 0, 120, 24);

        app.handle_mouse(mouse_event(MouseEventKind::ScrollDown, 10, 10), area);
        assert_eq!(app.list_selected(), 0);
    }

    #[test]
    fn mouse_click_switches_tabs() {
        let mut app = App::new_with_mouse(
            ThemeName::Aurora,
            true,
            false,
            true,
            KeyBindings::default(),
            true,
            DemoPanel::Overview,
        );
        let area = Rect::new(0, 0, 120, 24);
        let regions = MouseRegions::for_area(area).expect("regions");

        app.handle_mouse(
            mouse_event(
                MouseEventKind::Down(MouseButton::Left),
                regions.tabs_x + regions.tabs_width.saturating_sub(1),
                regions.tabs_row,
            ),
            area,
        );
        assert_eq!(app.panel(), DemoPanel::List);

        app.handle_mouse(
            mouse_event(
                MouseEventKind::Down(MouseButton::Left),
                regions.tabs_x,
                regions.tabs_row,
            ),
            area,
        );
        assert_eq!(app.panel(), DemoPanel::Overview);
    }

    #[test]
    fn mouse_click_selects_visible_list_row() {
        let mut app = App::new_with_mouse(
            ThemeName::Aurora,
            true,
            false,
            true,
            KeyBindings::default(),
            true,
            DemoPanel::List,
        );
        app.list_selected = 8;

        let area = Rect::new(0, 0, 120, 24);
        let regions = MouseRegions::for_area(area).expect("regions");
        let target_offset = 3_u16.min(regions.list_area.height.saturating_sub(1));
        let target_row = regions.list_area.y + target_offset;

        app.handle_mouse(
            mouse_event(
                MouseEventKind::Down(MouseButton::Left),
                regions.list_area.x,
                target_row,
            ),
            area,
        );

        let start = list_viewport_start(8, app.list_len(), regions.list_area.height as usize);
        assert_eq!(
            app.list_selected(),
            (start + target_offset as usize).min(app.list_len() - 1)
        );
    }
}
