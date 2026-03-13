use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::config::theme::Theme;
use crate::ui::keyboard::KeyboardWidget;

pub fn draw(frame: &mut Frame, app: &App, theme: &Theme, elapsed: std::time::Duration, effect_enabled: bool, palette: &[(u8, u8, u8)]) {
    let chunks = Layout::vertical([
        Constraint::Min(0),    // Top spacer
        Constraint::Length(1), // Info bar
        Constraint::Length(1), // Padding
        Constraint::Length(1), // Word queue
        Constraint::Length(1), // Current word
        Constraint::Length(1), // Input text
        Constraint::Length(1), // Spacer
        Constraint::Length(KeyboardWidget::required_height()),
        Constraint::Min(0),    // Bottom spacer
        Constraint::Length(1), // Help
    ])
    .split(frame.area());

    // Info bar: word count (left) and timer (right)
    let elapsed_secs = app.elapsed_secs();
    let minutes = (elapsed_secs as u64) / 60;
    let seconds = (elapsed_secs as u64) % 60;
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
    let mut input_chars = app.typing.input.chars();
    for expected in app.typing.current_word.chars() {
        let style = if let Some(typed) = input_chars.next() {
            if typed == expected {
                Style::default().fg(theme.word.correct())
            } else {
                Style::default().fg(theme.word.incorrect())
            }
        } else {
            Style::default().fg(theme.word.current())
        };
        spans.push(Span::styled(expected.to_string(), style));
    }

    let word_display = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(word_display, chunks[4]);

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
        .style(Style::default().fg(theme.word.queue()));
    frame.render_widget(queue, chunks[3]);

    // Input text: fixed-width underlined field matching current word length
    let word_char_count = app.typing.current_word.chars().count();
    let mut input_spans = Vec::new();
    let underline = Style::default()
        .fg(theme.word.current())
        .add_modifier(ratatui::style::Modifier::UNDERLINED);
    let mut word_chars = app.typing.current_word.chars();
    let mut typed_count = 0;
    for ch in app.typing.input.chars() {
        if typed_count < word_char_count {
            let expected = word_chars.next();
            let style = if expected == Some(ch) {
                underline.fg(theme.word.correct())
            } else {
                underline.fg(theme.word.incorrect())
            };
            input_spans.push(Span::styled(ch.to_string(), style));
        } else {
            input_spans.push(Span::styled(
                ch.to_string(),
                underline.fg(theme.word.incorrect()),
            ));
        }
        typed_count += 1;
    }
    // Fill remaining positions with underlined spaces
    for _ in typed_count..word_char_count {
        input_spans.push(Span::styled(" ", underline));
    }
    let input_display = Paragraph::new(Line::from(input_spans)).alignment(Alignment::Center);
    frame.render_widget(input_display, chunks[5]);

    // Keyboard
    let active_keys = app.available_keys();
    let kbd = KeyboardWidget::new(app.layout(), &active_keys, theme, elapsed, effect_enabled, palette);
    frame.render_widget(kbd, centered_rect(chunks[7], 75));

    // Help
    let dim = theme.dim_style();
    let key = theme.key_style();
    let remap_span = if app.remap.qwerty_remap {
        Span::styled("ON", key)
    } else {
        Span::styled("OFF", dim)
    };
    let help = Paragraph::new(Line::from(vec![
        Span::styled(" Esc", key),
        Span::styled(" Back | ", dim),
        Span::styled("Ctrl+T", key),
        Span::styled(" Remap: ", dim),
        remap_span,
    ]));
    frame.render_widget(help, chunks[9]);
}

fn centered_rect(area: Rect, width: u16) -> Rect {
    let width = width.min(area.width);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    Rect::new(x, area.y, width, area.height)
}
