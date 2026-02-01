use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.size();
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
    draw_footer(frame, layout[2], &theme);

    if app.show_help {
        draw_help(frame, area, &theme);
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

    let info = Line::from(vec![
        Span::styled(
            format!("Theme: {}", app.current_theme_name()),
            base.fg(theme.palette.fg),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("High contrast: {}", on_off(app.high_contrast)),
            base.fg(theme.palette.muted),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("No color: {}", on_off(app.no_color)),
            base.fg(theme.palette.muted),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Reduced motion: {}", on_off(app.reduced_motion)),
            base.fg(theme.palette.muted),
        ),
    ]);

    let header = Paragraph::new(Text::from(vec![title, info]))
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

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(area);

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

    frame.render_widget(commands, columns[0]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(columns[1]);

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
        Line::from(Span::raw("Press t to cycle themes.")),
    ]))
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title(" Theme ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.muted)),
    )
    .style(base);

    frame.render_widget(theme_info, right[0]);

    let accessibility = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "h: high contrast",
            base.fg(theme.palette.accent),
        )),
        Line::from("c: toggle color"),
        Line::from("r: reduced motion"),
        Line::from("?: help panel"),
        Line::from("q/esc: quit"),
    ]))
    .block(
        Block::default()
            .title(" Accessibility ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.palette.muted)),
    )
    .style(base);

    frame.render_widget(accessibility, right[1]);
}

fn draw_footer(frame: &mut Frame, area: Rect, theme: &crate::theme::Theme) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);

    let footer = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("Press ? for help.", base.fg(theme.palette.muted)),
        Span::raw(" "),
        Span::styled("Use q or esc to exit.", base.fg(theme.palette.danger)),
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

fn draw_help(frame: &mut Frame, area: Rect, theme: &crate::theme::Theme) {
    let base = Style::default().fg(theme.palette.fg).bg(theme.palette.bg);
    let popup_area = centered_rect(70, 60, area);

    let help_text = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            "Keys",
            base.fg(theme.palette.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from("t: cycle theme"),
        Line::from("h: toggle high contrast"),
        Line::from("c: toggle color"),
        Line::from("r: toggle reduced motion"),
        Line::from("?: toggle help"),
        Line::from("q/esc: quit"),
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

fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(rect);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    horizontal[1]
}
