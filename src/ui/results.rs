use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::config::theme::Theme;

pub fn draw(frame: &mut Frame, app: &App, theme: &Theme) {
    let elapsed = app.elapsed_secs();
    let wpm = if elapsed > 0.0 {
        (app.stats.total_chars as f64 / 5.0) / (elapsed / 60.0)
    } else {
        0.0
    };
    let accuracy = if app.stats.total_chars > 0 {
        (app.stats.correct_chars as f64 / app.stats.total_chars as f64) * 100.0
    } else {
        0.0
    };
    let minutes = (elapsed as u64) / 60;
    let seconds = (elapsed as u64) % 60;

    let dim = Style::default().fg(theme.secondary());
    let val = Style::default().fg(Color::Reset);

    let chunks = Layout::vertical([
        Constraint::Min(0),    // Top spacer
        Constraint::Length(1), // Title
        Constraint::Length(1), // Blank
        Constraint::Length(1), // WPM
        Constraint::Length(1), // Accuracy
        Constraint::Length(1), // Time
        Constraint::Length(1), // Blank
        Constraint::Length(1), // Help
        Constraint::Min(0),    // Bottom spacer
    ])
    .split(frame.area());

    let title = Paragraph::new(Line::from(vec![
        Span::styled("── ", dim),
        Span::styled("Results", val),
        Span::styled(" ──", dim),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(title, chunks[1]);

    let wpm_line = Paragraph::new(Line::from(vec![
        Span::styled("WPM        ", dim),
        Span::styled(format!("{:.0}", wpm), val),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(wpm_line, chunks[3]);

    let acc_line = Paragraph::new(Line::from(vec![
        Span::styled("Accuracy   ", dim),
        Span::styled(format!("{:.1}%", accuracy), val),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(acc_line, chunks[4]);

    let time_line = Paragraph::new(Line::from(vec![
        Span::styled("Time       ", dim),
        Span::styled(format!("{}:{:02}", minutes, seconds), val),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(time_line, chunks[5]);

    let help = Paragraph::new(Line::from(vec![Span::styled(
        "Enter: Retry  Esc: Back",
        dim,
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(help, chunks[7]);
}
