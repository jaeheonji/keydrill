use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use unicode_width::UnicodeWidthChar;

use crate::app::{App, SelectPhase};
use crate::config::theme::Theme;

const TITLE_ART: &[&str] = &[
    "██╗  ██╗███████╗██╗   ██╗██████╗ ██████╗ ██╗██╗     ██╗     ",
    "██║ ██╔╝██╔════╝╚██╗ ██╔╝██╔══██╗██╔══██╗██║██║     ██║     ",
    "█████╔╝ █████╗   ╚████╔╝ ██║  ██║██████╔╝██║██║     ██║     ",
    "██╔═██╗ ██╔══╝    ╚██╔╝  ██║  ██║██╔══██╗██║██║     ██║     ",
    "██║  ██╗███████╗   ██║   ██████╔╝██║  ██║██║███████╗███████╗",
    "╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝╚══════╝",
];

const TITLE_ART_HEIGHT: u16 = TITLE_ART.len() as u16;
const BOX_WIDTH: u16 = 50;

const SLIDE_DURATION_MS: f64 = 800.0;

pub fn draw(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();
    let elapsed_ms = app.select.select_screen_entered_at.elapsed().as_secs_f64() * 1000.0;

    // Build the list items to determine box height
    let (items, title, help_text) = match app.select.select_phase {
        SelectPhase::Layout => build_layout_items(app, theme),
        SelectPhase::Level => build_level_items(app, theme),
    };

    let item_count = items.len() as u16;
    let box_height = item_count + 2; // +2 for border

    // Total content height: title art + 1 gap + box + 1 gap + help
    let content_height = TITLE_ART_HEIGHT + 1 + box_height + 1 + 1;

    // Vertical centering
    let [content_area] = Layout::vertical([Constraint::Length(content_height)])
        .flex(Flex::Center)
        .areas(area);

    let [title_area, _gap1, box_area, _gap2, help_area] = Layout::vertical([
        Constraint::Length(TITLE_ART_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(box_height),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(content_area);

    // ASCII art title — direct buffer rendering with animation
    render_title_animated(frame.buffer_mut(), title_area, elapsed_ms, theme.title());

    // Centered box
    let [box_centered] = Layout::horizontal([Constraint::Length(BOX_WIDTH)])
        .flex(Flex::Center)
        .areas(box_area);

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {title} ")),
    );
    frame.render_widget(list, box_centered);

    // Help text
    let help = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.secondary()));
    frame.render_widget(help, help_area);
}

/// Cubic ease-out: fast start, smooth deceleration
fn ease_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

fn render_title_animated(buf: &mut Buffer, area: Rect, elapsed_ms: f64, color: Color) {
    // Vertical slide-in: from below upward
    let slide_offset_y: i32 = if elapsed_ms < SLIDE_DURATION_MS {
        let progress = ease_out_cubic(elapsed_ms / SLIDE_DURATION_MS);
        let start_offset = TITLE_ART_HEIGHT as f64 + 2.0;
        (start_offset * (1.0 - progress)) as i32
    } else {
        0
    };

    for (row_idx, line) in TITLE_ART.iter().enumerate() {
        let target_y = area.y as i32 + row_idx as i32 + slide_offset_y;
        if target_y < area.y as i32 || target_y >= (area.y + area.height) as i32 {
            continue;
        }
        let y = target_y as u16;

        // Per-line display width for centering
        let line_width: usize = line.chars().map(|c| c.width().unwrap_or(0).max(1)).sum();
        let line_x = if (area.width as usize) >= line_width {
            area.x + ((area.width as usize - line_width) / 2) as u16
        } else {
            area.x
        };

        let mut col: usize = 0;
        for ch in line.chars() {
            let ch_width = ch.width().unwrap_or(0);
            if ch_width == 0 && ch != ' ' {
                continue;
            }

            let x = line_x + col as u16;
            let cw = ch_width.max(1);

            if x + cw as u16 <= area.x + area.width {
                let cell = &mut buf[(x, y)];
                cell.set_char(ch);
                cell.set_fg(color);
            }

            col += cw;
        }
    }
}

fn build_layout_items<'a>(
    app: &App,
    theme: &Theme,
) -> (Vec<ListItem<'a>>, &'static str, &'static str) {
    let items: Vec<ListItem> = app
        .layouts
        .iter()
        .enumerate()
        .map(|(i, layout)| {
            let selected = i == app.select.current_layout_idx;
            let prefix = if selected { "▸ " } else { "  " };
            let content = format!("{prefix}{}", layout.name);
            let style = if selected {
                Style::default()
                    .fg(theme.selected())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Reset)
            };
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    (
        items,
        "Select Layout",
        "↑↓ Navigate  Enter/→ Select  Esc Quit",
    )
}

fn build_level_items<'a>(
    app: &App,
    theme: &Theme,
) -> (Vec<ListItem<'a>>, &'static str, &'static str) {
    let layout = app.layout();
    let items: Vec<ListItem> = layout
        .levels
        .iter()
        .enumerate()
        .map(|(i, level)| {
            let keys_str: String = level.new_keys.iter().collect();
            let selected = i == app.select.current_level;
            let prefix = if selected { "▸ " } else { "  " };
            let content = format!("{prefix}{}. {} [{keys_str}]", i + 1, level.name);
            let style = if selected {
                Style::default()
                    .fg(theme.selected())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Reset)
            };
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let help = if app.layouts.len() > 1 {
        "↑↓ Navigate  Enter/→ Select  ←/Esc Back"
    } else {
        "↑↓ Navigate  Enter/→ Select  Esc Quit"
    };

    (items, "Select Level", help)
}
