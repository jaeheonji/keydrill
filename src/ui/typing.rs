use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::config::theme::Theme;
use crate::ui::keyboard::KeyboardWidget;

pub fn draw(frame: &mut Frame, app: &App, theme: &Theme) {
    let chunks = Layout::vertical([
        Constraint::Min(0),    // Top spacer
        Constraint::Length(1), // Info bar
        Constraint::Length(1), // Padding
        Constraint::Length(1), // Current word
        Constraint::Length(1), // Word queue
        Constraint::Length(1), // Spacer
        Constraint::Length(KeyboardWidget::required_height()),
        Constraint::Min(0),    // Bottom spacer
        Constraint::Length(1), // Help
    ])
    .split(frame.area());

    // Info bar: word count (left) and timer (right)
    let elapsed = app.elapsed_secs();
    let minutes = (elapsed as u64) / 60;
    let seconds = (elapsed as u64) % 60;
    let info_bar = Paragraph::new(Line::from(vec![Span::styled(
        format!(
            "Word {}/{}  {:02}:{:02}",
            app.stats.completed_words + 1,
            app.stats.total_words,
            minutes,
            seconds
        ),
        Style::default().fg(theme.secondary()),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(info_bar, chunks[1]);

    // Current word with per-character coloring
    let mut spans = Vec::new();
    for (i, expected) in app.typing.current_word.chars().enumerate() {
        let style = if let Some(typed) = app.typing.input.chars().nth(i) {
            if typed == expected {
                Style::default().fg(theme.correct())
            } else {
                Style::default().fg(theme.incorrect()).bg(theme.secondary())
            }
        } else {
            Style::default().fg(Color::Reset)
        };
        spans.push(Span::styled(expected.to_string(), style));
    }

    let word_display = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(word_display, chunks[3]);

    // Word queue preview
    let queue_preview: String = app
        .typing
        .word_queue
        .iter()
        .take(10)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("  ");
    let queue = Paragraph::new(queue_preview)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.secondary()));
    frame.render_widget(queue, chunks[4]);

    // Keyboard
    let active_keys = app.available_keys();
    let highlight = app.next_expected_char();
    let kbd = KeyboardWidget::new(app.layout(), &active_keys, highlight, theme);
    frame.render_widget(kbd, centered_rect(chunks[6], 75));

    // Help
    let dim = Style::default().fg(theme.secondary());
    let remap_span = if app.remap.qwerty_remap {
        Span::styled("ON", Style::default().fg(theme.correct()))
    } else {
        Span::styled("OFF", dim)
    };
    let help = Paragraph::new(Line::from(vec![
        Span::styled(" Esc Back | Ctrl+T Remap: ", dim),
        remap_span,
    ]));
    frame.render_widget(help, chunks[8]);
}

fn centered_rect(area: Rect, width: u16) -> Rect {
    let width = width.min(area.width);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    Rect::new(x, area.y, width, area.height)
}
