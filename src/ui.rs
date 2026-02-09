use crate::app::App;
use crate::keys;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{
    Block, Borders, Cell, Clear, Gauge, List, ListItem, Paragraph, Row, Table, Wrap,
};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let theme = app.current_theme();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // Keep room for a title row + at least one status row.
            Constraint::Length(4),
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

    let showcase_block = Block::default()
        .title(" Showcase ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.palette.muted))
        .style(base);

    let accessibility = accessibility_panel(area, app, base, theme);

    if is_narrow(area) {
        let stack = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(0),
                Constraint::Length(4),
            ])
            .split(area);

        frame.render_widget(commands, stack[0]);
        frame.render_widget(showcase_block.clone(), stack[1]);
        draw_showcase_contents(frame, showcase_block, stack[1], app, theme);
        frame.render_widget(accessibility, stack[2]);
    } else {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
            .split(area);

        frame.render_widget(commands, columns[0]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(columns[1]);

        frame.render_widget(showcase_block.clone(), right[0]);
        draw_showcase_contents(frame, showcase_block, right[0], app, theme);
        frame.render_widget(accessibility, right[1]);
    }
}

fn draw_showcase_contents(
    frame: &mut Frame,
    block: Block<'_>,
    area: Rect,
    app: &App,
    theme: &crate::theme::Theme,
) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);
    let inner = block.inner(area);

    if inner.width <= 2 || inner.height <= 2 {
        return;
    }

    // Keep this stable and non-"animated": the TUI template should demonstrate patterns even in
    // `demo --no-tty` static previews.
    let progress_ratio = 0.62_f64;
    let progress_percent = (progress_ratio * 100.0).round() as u16;

    let theme_lines = if inner.height >= 8 { 3 } else { 2 };
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(theme_lines),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    let mut theme_info_lines = vec![
        Line::from(Span::styled(
            app.current_theme_name(),
            base.fg(theme.palette.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            app.current_theme_description(),
            base.fg(theme.palette.muted),
        )),
    ];
    if theme_lines >= 3 {
        theme_info_lines.push(Line::from(Span::raw(format!(
            "Press {} to cycle themes.",
            keys::key_list_display(&app.keymap.cycle_theme)
        ))));
    }

    let theme_info = Paragraph::new(Text::from(theme_info_lines))
        .wrap(Wrap { trim: true })
        .style(base);

    let gauge = Gauge::default()
        .ratio(progress_ratio)
        .gauge_style(base.fg(theme.palette.accent))
        .label(format!("{progress_percent}%"));

    let rows = vec![
        Row::new(vec![
            Cell::from("cycle theme"),
            Cell::from(keys::key_list_display(&app.keymap.cycle_theme)),
        ]),
        Row::new(vec![
            Cell::from("help"),
            Cell::from(keys::key_list_display(&app.keymap.toggle_help)),
        ]),
        Row::new(vec![
            Cell::from("quit"),
            Cell::from(app.keymap.quit_label()),
        ]),
    ];

    let table = Table::new(
        rows,
        [Constraint::Percentage(55), Constraint::Percentage(45)],
    )
    .header(
        Row::new(vec![Cell::from("Action"), Cell::from("Key")])
            .style(base.fg(theme.palette.muted).add_modifier(Modifier::BOLD)),
    )
    .column_spacing(1)
    .style(base);

    frame.render_widget(theme_info, layout[0]);
    frame.render_widget(gauge, layout[1]);
    frame.render_widget(table, layout[2]);
}

fn accessibility_panel(
    area: Rect,
    app: &App,
    base: Style,
    theme: &crate::theme::Theme,
) -> Paragraph<'static> {
    // In narrow/short layouts, prefer a compact one-liner (help overlay contains the full list).
    if area.width < 90 || area.height < 20 {
        let compact = Line::from(vec![
            Span::styled(
                "Keys: ",
                base.fg(theme.palette.muted).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{} contrast | {} color | {} motion | {} help | {} quit",
                keys::key_list_display(&app.keymap.toggle_high_contrast),
                keys::key_list_display(&app.keymap.toggle_color),
                keys::key_list_display(&app.keymap.toggle_reduced_motion),
                keys::key_list_display(&app.keymap.toggle_help),
                app.keymap.quit_label()
            )),
        ]);

        return Paragraph::new(Text::from(compact))
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" Accessibility ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.palette.muted)),
            )
            .style(base);
    }

    Paragraph::new(Text::from(vec![
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
    .style(base)
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
        format!("High contrast: {}", on_off(app.high_contrast)),
        base.fg(theme.palette.muted),
    );

    let no_color = Span::styled(
        format!("No color: {}", on_off(app.no_color)),
        base.fg(theme.palette.muted),
    );

    let reduced_motion = Span::styled(
        format!("Reduced motion: {}", on_off(app.reduced_motion)),
        base.fg(theme.palette.muted),
    );

    if area.width < 70 {
        vec![Line::from(vec![
            theme_label,
            Span::raw(" | "),
            no_color,
            Span::raw(" | "),
            reduced_motion,
        ])]
    } else {
        vec![Line::from(vec![
            theme_label,
            Span::raw(" | "),
            high_contrast,
            Span::raw(" | "),
            no_color,
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

pub fn render_static_preview(app: &App, width: u16, height: u16) -> anyhow::Result<String> {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|frame| draw(frame, app))?;

    let buffer = terminal.backend().buffer();
    let mut lines = Vec::<String>::with_capacity(height as usize);

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            line.push_str(buffer[(x, y)].symbol());
        }
        lines.push(line.trim_end_matches(' ').to_string());
    }

    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }

    Ok(lines.join("\n") + "\n")
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
        let y_showcase = find_row(&lines, " Showcase ").expect("showcase title");
        let y_access = find_row(&lines, " Accessibility ").expect("access title");

        assert!(y_commands < y_showcase);
        assert!(y_showcase < y_access);
    }

    #[test]
    fn wide_layout_places_commands_left_and_panels_right() {
        let app = App::new(ThemeName::Aurora, true, false, true, KeyBindings::default());

        let lines = render_lines(120, 24, &app);
        let y_commands = find_row(&lines, " Commands ").expect("commands title");
        let y_showcase = find_row(&lines, " Showcase ").expect("showcase title");
        let y_access = find_row(&lines, " Accessibility ").expect("access title");

        assert!((y_commands as i32 - y_showcase as i32).abs() <= 1);
        assert!(y_access > y_showcase);
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
