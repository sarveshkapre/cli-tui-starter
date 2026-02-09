use crate::app::App;
use crate::keys;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let theme = app.current_theme();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    draw_header(frame, layout[0], app, &theme);
    draw_body(frame, layout[1], app, &theme);
    draw_footer(frame, layout[2], app, &theme);

    if app.show_help {
        draw_help(frame, area, app, &theme);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);

    let title = Line::from(vec![
        Span::styled(
            " CLI TUI Starter ",
            base.fg(theme.palette.accent).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(app.spinner_frame(), base.fg(theme.palette.muted)),
        Span::raw(" "),
        Span::styled("ready", base.fg(theme.palette.success)),
    ]);

    let info_lines = header_info_lines(area, app, base, theme);

    let mut lines = Vec::with_capacity(1 + info_lines.len());
    lines.push(title);
    lines.extend(info_lines);

    let header = Paragraph::new(Text::from(lines))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.palette.muted)),
        )
        .style(base);

    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);

    let commands = List::new(vec![
        ListItem::new("cli-tui-starter demo --theme aurora"),
        ListItem::new("cli-tui-starter themes"),
        ListItem::new("cli-tui-starter keys"),
    ])
    .block(
        Block::default()
            .title(" Commands ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.muted)),
    )
    .style(base);

    let theme_info = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            app.current_theme_name(),
            base.fg(theme.palette.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            app.current_theme_description(),
            base.fg(theme.palette.muted),
        )),
        Line::from(""),
        Line::from(Span::raw(format!(
            "Press {} to cycle themes.",
            keys::key_list_display(&app.keymap.cycle_theme)
        ))),
    ]))
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(" Theme ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.muted)),
    )
    .style(base);

    let accessibility = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            format!(
                "{}: high contrast",
                keys::key_list_display(&app.keymap.toggle_high_contrast)
            ),
            base.fg(theme.palette.accent),
        )),
        Line::from(format!(
            "{}: toggle color",
            keys::key_list_display(&app.keymap.toggle_color)
        )),
        Line::from(format!(
            "{}: reduced motion",
            keys::key_list_display(&app.keymap.toggle_reduced_motion)
        )),
        Line::from(format!(
            "{}: help panel",
            keys::key_list_display(&app.keymap.toggle_help)
        )),
        Line::from(format!("{}: quit", app.keymap.quit_label())),
    ]))
    .block(
        Block::default()
            .title(" Accessibility ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.muted)),
    )
    .style(base);

    if is_narrow(area) {
        let stack = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Min(0),
            ])
            .split(area);

        frame.render_widget(commands, stack[0]);
        frame.render_widget(theme_info, stack[1]);
        frame.render_widget(accessibility, stack[2]);
    } else {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
            .split(area);

        frame.render_widget(commands, columns[0]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(columns[1]);

        frame.render_widget(theme_info, right[0]);
        frame.render_widget(accessibility, right[1]);
    }
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);

    let footer = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(
            format!(
                "Press {} for help.",
                keys::key_list_display(&app.keymap.toggle_help)
            ),
            base.fg(theme.palette.muted),
        ),
        Span::raw(" "),
        Span::styled(
            format!("Use {} to exit.", app.keymap.quit_label()),
            base.fg(theme.palette.danger),
        ),
    ])))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.muted)),
    )
    .style(base);

    frame.render_widget(footer, area);
}

fn draw_help(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);
    let popup_area = centered_popup_rect(area);

    let help_text = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "Keys",
            base.fg(theme.palette.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(format!(
            "{}: cycle theme",
            keys::key_list_display(&app.keymap.cycle_theme)
        )),
        Line::from(format!(
            "{}: toggle high contrast",
            keys::key_list_display(&app.keymap.toggle_high_contrast)
        )),
        Line::from(format!(
            "{}: toggle color",
            keys::key_list_display(&app.keymap.toggle_color)
        )),
        Line::from(format!(
            "{}: toggle reduced motion",
            keys::key_list_display(&app.keymap.toggle_reduced_motion)
        )),
        Line::from(format!(
            "{}: toggle help",
            keys::key_list_display(&app.keymap.toggle_help)
        )),
        Line::from(format!("{}: quit", app.keymap.quit_label())),
        Line::from(""),
        Line::from(Span::styled(
            "Accessibility",
            base.fg(theme.palette.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from("- No-color mode for screen readers"),
        Line::from("- High-contrast palette"),
        Line::from("- Reduced motion toggle"),
    ]))
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.accent)),
    )
    .style(base);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(help_text, popup_area);
}

fn on_off(value: bool) -> &'static str {
    if value {
        "on"
    } else {
        "off"
    }
}

fn is_narrow(area: Rect) -> bool {
    area.width < 90
}

fn header_info_lines(
    area: Rect,
    app: &App,
    base: Style,
    theme: &crate::theme::Theme,
) -> Vec<Line<'static>> {
    let theme_label = Span::styled(
        format!("Theme: {}", app.current_theme_name()),
        base.fg(theme.palette.fg),
    );

    let high_contrast = Span::styled(
        format!("HC: {}", on_off(app.high_contrast)),
        base.fg(theme.palette.muted),
    );

    let color = Span::styled(
        format!("Color: {}", on_off(!app.no_color)),
        base.fg(theme.palette.muted),
    );

    let reduced_motion = Span::styled(
        format!("Motion: {}", on_off(app.reduced_motion)),
        base.fg(theme.palette.muted),
    );

    if area.width < 62 {
        vec![
            Line::from(vec![theme_label, Span::raw(" | "), high_contrast]),
            Line::from(vec![color, Span::raw(" | "), reduced_motion]),
        ]
    } else {
        vec![Line::from(vec![
            theme_label,
            Span::raw(" | "),
            high_contrast,
            Span::raw(" | "),
            color,
            Span::raw(" | "),
            reduced_motion,
        ])]
    }
}

fn centered_popup_rect(area: Rect) -> Rect {
    if area.width <= 2 || area.height <= 2 {
        return area;
    }

    let max_width = area.width.saturating_sub(4);
    let max_height = area.height.saturating_sub(2);

    let width = max_width.clamp(20, 84);
    let height = max_height.clamp(10, 16);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::cli::ThemeName;
    use crate::keys::{parse_key_spec, KeyBindings};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn render_lines(width: u16, height: u16, app: &App) -> Vec<String> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal.draw(|f| super::draw(f, app)).expect("draw");

        let buf = terminal.backend().buffer().clone();
        (0..height)
            .map(|y| {
                let mut line = String::new();
                for x in 0..width {
                    line.push_str(buf[(x, y)].symbol());
                }
                line
            })
            .collect()
    }

    fn find_row(lines: &[String], needle: &str) -> Option<usize> {
        lines.iter().position(|l| l.contains(needle))
    }

    #[test]
    fn narrow_layout_stacks_sections_vertically() {
        let app = App::new(
            ThemeName::Aurora,
            true,  // no_color for stable rendering
            false, // high_contrast
            true,  // reduced_motion
            KeyBindings::default(),
        );

        let lines = render_lines(80, 24, &app);
        let y_commands = find_row(&lines, " Commands ").expect("commands title");
        let y_theme = find_row(&lines, " Theme ").expect("theme title");
        let y_access = find_row(&lines, " Accessibility ").expect("access title");

        assert!(y_commands < y_theme);
        assert!(y_theme < y_access);
    }

    #[test]
    fn wide_layout_places_commands_left_and_panels_right() {
        let app = App::new(ThemeName::Aurora, true, false, true, KeyBindings::default());

        let lines = render_lines(120, 24, &app);
        let y_commands = find_row(&lines, " Commands ").expect("commands title");
        let y_theme = find_row(&lines, " Theme ").expect("theme title");
        let y_access = find_row(&lines, " Accessibility ").expect("access title");

        assert!((y_commands as i32 - y_theme as i32).abs() <= 1);
        assert!(y_access > y_theme);
    }

    #[test]
    fn help_panel_uses_active_keymap_labels() {
        let keymap = KeyBindings {
            cycle_theme: vec![parse_key_spec("n").unwrap()],
            toggle_help: vec![parse_key_spec("!").unwrap()],
            ..KeyBindings::default()
        };
        keymap.validate().unwrap();

        let mut app = App::new(ThemeName::Aurora, true, false, true, keymap);
        app.show_help = true;

        let lines = render_lines(90, 24, &app);
        let merged = lines.join("\n");

        assert!(merged.contains("n: cycle theme"));
        assert!(merged.contains("!: toggle help"));
    }
}
