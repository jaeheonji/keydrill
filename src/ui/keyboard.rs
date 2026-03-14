use std::time::Duration;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::config::theme::Theme;
use crate::layout::Layout;
use crate::ui::color_cycle;

const KEY_HEIGHT: u16 = 3;

struct PhysicalKey {
    row: u8,
    x: u16,
    width: u16,
    label: &'static str,
    /// Maps to (row, col) in the Layout's key definitions.
    /// None for decorative/modifier keys.
    grid_pos: Option<(u8, u8)>,
}

macro_rules! key {
    // Typeable key: grid_pos = Some((r, c)), label = ""
    ($row:expr, $x:expr, $w:expr, ($r:expr, $c:expr)) => {
        PhysicalKey { row: $row, x: $x, width: $w, label: "", grid_pos: Some(($r, $c)) }
    };
    // Modifier/decorator key: grid_pos = None, label = $lbl
    ($row:expr, $x:expr, $w:expr, $lbl:expr) => {
        PhysicalKey { row: $row, x: $x, width: $w, label: $lbl, grid_pos: None }
    };
}

// Full ANSI 60% keyboard layout
// 1u = 5 chars, total width per row = 75 chars
const PHYSICAL_KEYS: &[PhysicalKey] = &[
    // Row 0: 13×1u + Backspace(2u=10) = 75
    key!(0,  0, 5, (0, 0)),
    key!(0,  5, 5, (0, 1)),
    key!(0, 10, 5, (0, 2)),
    key!(0, 15, 5, (0, 3)),
    key!(0, 20, 5, (0, 4)),
    key!(0, 25, 5, (0, 5)),
    key!(0, 30, 5, (0, 6)),
    key!(0, 35, 5, (0, 7)),
    key!(0, 40, 5, (0, 8)),
    key!(0, 45, 5, (0, 9)),
    key!(0, 50, 5, (0, 10)),
    key!(0, 55, 5, (0, 11)),
    key!(0, 60, 5, (0, 12)),
    key!(0, 65, 10, "Bksp"),
    // Row 1: Tab(8) + 12×1u + \(7) = 75
    key!(1,  0, 8, "Tab"),
    key!(1,  8, 5, (1, 0)),
    key!(1, 13, 5, (1, 1)),
    key!(1, 18, 5, (1, 2)),
    key!(1, 23, 5, (1, 3)),
    key!(1, 28, 5, (1, 4)),
    key!(1, 33, 5, (1, 5)),
    key!(1, 38, 5, (1, 6)),
    key!(1, 43, 5, (1, 7)),
    key!(1, 48, 5, (1, 8)),
    key!(1, 53, 5, (1, 9)),
    key!(1, 58, 5, (1, 10)),
    key!(1, 63, 5, (1, 11)),
    key!(1, 68, 7, "\\"),
    // Row 2: Caps(9) + 11×1u + Enter(11) = 75
    key!(2,  0, 9, "Caps"),
    key!(2,  9, 5, (2, 0)),
    key!(2, 14, 5, (2, 1)),
    key!(2, 19, 5, (2, 2)),
    key!(2, 24, 5, (2, 3)),
    key!(2, 29, 5, (2, 4)),
    key!(2, 34, 5, (2, 5)),
    key!(2, 39, 5, (2, 6)),
    key!(2, 44, 5, (2, 7)),
    key!(2, 49, 5, (2, 8)),
    key!(2, 54, 5, (2, 9)),
    key!(2, 59, 5, (2, 10)),
    key!(2, 64, 11, "Enter"),
    // Row 3: LShift(11) + 10×1u + RShift(14) = 75
    key!(3,  0, 11, "Shift"),
    key!(3, 11, 5, (3, 0)),
    key!(3, 16, 5, (3, 1)),
    key!(3, 21, 5, (3, 2)),
    key!(3, 26, 5, (3, 3)),
    key!(3, 31, 5, (3, 4)),
    key!(3, 36, 5, (3, 5)),
    key!(3, 41, 5, (3, 6)),
    key!(3, 46, 5, (3, 7)),
    key!(3, 51, 5, (3, 8)),
    key!(3, 56, 5, (3, 9)),
    key!(3, 61, 14, "Shift"),
    // Row 4: Ctrl(8)+Sup(7)+Alt(7)+Space(25)+Alt(7)+Sup(7)+Fn(6)+Ctrl(8) = 75
    key!(4,  0,  8, "Ctrl"),
    key!(4,  8,  7, "Sup"),
    key!(4, 15,  7, "Alt"),
    key!(4, 22, 25, ""),
    key!(4, 47,  7, "Alt"),
    key!(4, 54,  7, "Sup"),
    key!(4, 61,  6, "Fn"),
    key!(4, 67,  8, "Ctrl"),
];

pub struct KeyboardWidget<'a> {
    layout: &'a Layout,
    active_keys: &'a [char],
    theme: &'a Theme,
    elapsed: Duration,
    effect_enabled: bool,
    palette: &'a [(u8, u8, u8)],
}

impl<'a> KeyboardWidget<'a> {
    pub fn new(
        layout: &'a Layout,
        active_keys: &'a [char],
        theme: &'a Theme,
        elapsed: Duration,
        effect_enabled: bool,
        palette: &'a [(u8, u8, u8)],
    ) -> Self {
        Self {
            layout,
            active_keys,
            theme,
            elapsed,
            effect_enabled,
            palette,
        }
    }

    pub fn required_height() -> u16 {
        KEY_HEIGHT * 5
    }
}

impl Widget for KeyboardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inactive_style = Style::default().fg(self.theme.keyboard.inactive.border());
        let inactive_text = Style::default().fg(self.theme.keyboard.inactive.text());
        let active_border_style = Style::default().fg(self.theme.keyboard.active.border());
        let active_text_style = Style::default().fg(self.theme.keyboard.active.text());

        for pk in PHYSICAL_KEYS {
            let x = area.x + pk.x;
            let y = area.y + pk.row as u16 * KEY_HEIGHT;

            if x + pk.width > area.right() || y + KEY_HEIGHT > area.bottom() {
                continue;
            }

            let rect = Rect::new(x, y, pk.width, KEY_HEIGHT);

            if let Some((row, col)) = pk.grid_pos {
                let layout_key = self
                    .layout
                    .keys
                    .iter()
                    .find(|k| k.position.row == row && k.position.col == col);

                if let Some(key) = layout_key {
                    let is_active = self.active_keys.contains(&key.normal);
                    let mut char_buf = [0u8; 4];
                    let label = key.normal.encode_utf8(&mut char_buf);

                    if is_active {
                        if self.effect_enabled {
                            draw_key_box_animated(buf, rect, self.elapsed, self.palette);
                            let color = color_cycle::cycling_color(self.elapsed, self.palette);
                            draw_key_label(buf, rect, label, Style::default().fg(color));
                        } else {
                            draw_key_border(buf, rect, |_, _| active_border_style);
                            draw_key_label(buf, rect, label, active_text_style);
                        }
                    } else {
                        draw_key_border(buf, rect, |_, _| inactive_style);
                        draw_key_label(buf, rect, label, inactive_text);
                    }
                } else {
                    draw_key_border(buf, rect, |_, _| inactive_style);
                }
            } else {
                draw_key_border(buf, rect, |_, _| inactive_style);
                draw_key_label(buf, rect, pk.label, inactive_text);
            }
        }
    }
}

/// Draw only the label (centered) in a key box, overwriting existing cells.
fn draw_key_label(buf: &mut Buffer, rect: Rect, label: &str, style: Style) {
    if label.is_empty() {
        return;
    }
    let buf_right = buf.area().right();
    let buf_bottom = buf.area().bottom();
    let label_y = rect.y + rect.height / 2;
    if label_y >= buf_bottom {
        return;
    }
    let interior = rect.width.saturating_sub(2) as usize;
    let label_len = label.chars().count();
    let pad = interior.saturating_sub(label_len) / 2;
    let start_x = rect.x + 1 + pad as u16;

    for (i, ch) in label.chars().enumerate() {
        let lx = start_x + i as u16;
        if lx < buf_right && lx < rect.x + rect.width - 1 {
            buf[(lx, label_y)].set_char(ch).set_style(style);
        }
    }
}

fn draw_key_box_animated(buf: &mut Buffer, rect: Rect, elapsed: Duration, palette: &[(u8, u8, u8)]) {
    let w = rect.width.saturating_sub(1) as f32;
    let h = rect.height.saturating_sub(1) as f32;
    let perimeter = 2.0 * (w + h);
    if perimeter == 0.0 {
        return;
    }

    draw_key_border(buf, rect, |dx, dy| {
        let pos = if dx == 0 {
            dy as f32
        } else if dy == rect.height - 1 {
            h + dx as f32
        } else if dx == rect.width - 1 {
            h + w + (h - dy as f32)
        } else {
            2.0 * h + w + (w - dx as f32)
        };
        let normalized = pos / perimeter;
        Style::default().fg(color_cycle::border_color(elapsed, normalized, palette))
    });
}

fn draw_key_border(buf: &mut Buffer, rect: Rect, style_at: impl Fn(u16, u16) -> Style) {
    let buf_right = buf.area().right();
    let buf_bottom = buf.area().bottom();

    // Top border
    if rect.y < buf_bottom {
        for dx in 1..rect.width.saturating_sub(1) {
            let x = rect.x + dx;
            if x < buf_right {
                buf[(x, rect.y)].set_char('─').set_style(style_at(dx, 0));
            }
        }
    }

    // Bottom border
    let bot = rect.y + rect.height - 1;
    if bot < buf_bottom {
        for dx in 1..rect.width.saturating_sub(1) {
            let x = rect.x + dx;
            if x < buf_right {
                buf[(x, bot)].set_char('─').set_style(style_at(dx, rect.height - 1));
            }
        }
    }

    // Side borders
    for dy in 1..rect.height.saturating_sub(1) {
        let y = rect.y + dy;
        if y < buf_bottom {
            if rect.x < buf_right {
                buf[(rect.x, y)].set_char('│').set_style(style_at(0, dy));
            }
            let right = rect.x + rect.width - 1;
            if right < buf_right {
                buf[(right, y)].set_char('│').set_style(style_at(rect.width - 1, dy));
            }
        }
    }

    // Corners
    let corners = [
        (0u16, 0u16, '┌'),
        (rect.width - 1, 0, '┐'),
        (0, rect.height - 1, '└'),
        (rect.width - 1, rect.height - 1, '┘'),
    ];
    for (dx, dy, c) in corners {
        let x = rect.x + dx;
        let y = rect.y + dy;
        if x < buf_right && y < buf_bottom {
            buf[(x, y)].set_char(c).set_style(style_at(dx, dy));
        }
    }
}
