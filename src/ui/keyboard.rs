use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::config::theme::Theme;
use crate::layout::Layout;

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

// Full ANSI 60% keyboard layout
// 1u = 5 chars, total width per row = 75 chars
const PHYSICAL_KEYS: &[PhysicalKey] = &[
    // Row 0: 13×1u + Backspace(2u=10) = 75
    PhysicalKey {
        row: 0,
        x: 0,
        width: 5,
        label: "",
        grid_pos: Some((0, 0)),
    },
    PhysicalKey {
        row: 0,
        x: 5,
        width: 5,
        label: "",
        grid_pos: Some((0, 1)),
    },
    PhysicalKey {
        row: 0,
        x: 10,
        width: 5,
        label: "",
        grid_pos: Some((0, 2)),
    },
    PhysicalKey {
        row: 0,
        x: 15,
        width: 5,
        label: "",
        grid_pos: Some((0, 3)),
    },
    PhysicalKey {
        row: 0,
        x: 20,
        width: 5,
        label: "",
        grid_pos: Some((0, 4)),
    },
    PhysicalKey {
        row: 0,
        x: 25,
        width: 5,
        label: "",
        grid_pos: Some((0, 5)),
    },
    PhysicalKey {
        row: 0,
        x: 30,
        width: 5,
        label: "",
        grid_pos: Some((0, 6)),
    },
    PhysicalKey {
        row: 0,
        x: 35,
        width: 5,
        label: "",
        grid_pos: Some((0, 7)),
    },
    PhysicalKey {
        row: 0,
        x: 40,
        width: 5,
        label: "",
        grid_pos: Some((0, 8)),
    },
    PhysicalKey {
        row: 0,
        x: 45,
        width: 5,
        label: "",
        grid_pos: Some((0, 9)),
    },
    PhysicalKey {
        row: 0,
        x: 50,
        width: 5,
        label: "",
        grid_pos: Some((0, 10)),
    },
    PhysicalKey {
        row: 0,
        x: 55,
        width: 5,
        label: "",
        grid_pos: Some((0, 11)),
    },
    PhysicalKey {
        row: 0,
        x: 60,
        width: 5,
        label: "",
        grid_pos: Some((0, 12)),
    },
    PhysicalKey {
        row: 0,
        x: 65,
        width: 10,
        label: "Bksp",
        grid_pos: None,
    },
    // Row 1: Tab(8) + 12×1u + \(7) = 75
    PhysicalKey {
        row: 1,
        x: 0,
        width: 8,
        label: "Tab",
        grid_pos: None,
    },
    PhysicalKey {
        row: 1,
        x: 8,
        width: 5,
        label: "",
        grid_pos: Some((1, 0)),
    },
    PhysicalKey {
        row: 1,
        x: 13,
        width: 5,
        label: "",
        grid_pos: Some((1, 1)),
    },
    PhysicalKey {
        row: 1,
        x: 18,
        width: 5,
        label: "",
        grid_pos: Some((1, 2)),
    },
    PhysicalKey {
        row: 1,
        x: 23,
        width: 5,
        label: "",
        grid_pos: Some((1, 3)),
    },
    PhysicalKey {
        row: 1,
        x: 28,
        width: 5,
        label: "",
        grid_pos: Some((1, 4)),
    },
    PhysicalKey {
        row: 1,
        x: 33,
        width: 5,
        label: "",
        grid_pos: Some((1, 5)),
    },
    PhysicalKey {
        row: 1,
        x: 38,
        width: 5,
        label: "",
        grid_pos: Some((1, 6)),
    },
    PhysicalKey {
        row: 1,
        x: 43,
        width: 5,
        label: "",
        grid_pos: Some((1, 7)),
    },
    PhysicalKey {
        row: 1,
        x: 48,
        width: 5,
        label: "",
        grid_pos: Some((1, 8)),
    },
    PhysicalKey {
        row: 1,
        x: 53,
        width: 5,
        label: "",
        grid_pos: Some((1, 9)),
    },
    PhysicalKey {
        row: 1,
        x: 58,
        width: 5,
        label: "",
        grid_pos: Some((1, 10)),
    },
    PhysicalKey {
        row: 1,
        x: 63,
        width: 5,
        label: "",
        grid_pos: Some((1, 11)),
    },
    PhysicalKey {
        row: 1,
        x: 68,
        width: 7,
        label: "\\",
        grid_pos: None,
    },
    // Row 2: Caps(9) + 11×1u + Enter(11) = 75
    PhysicalKey {
        row: 2,
        x: 0,
        width: 9,
        label: "Caps",
        grid_pos: None,
    },
    PhysicalKey {
        row: 2,
        x: 9,
        width: 5,
        label: "",
        grid_pos: Some((2, 0)),
    },
    PhysicalKey {
        row: 2,
        x: 14,
        width: 5,
        label: "",
        grid_pos: Some((2, 1)),
    },
    PhysicalKey {
        row: 2,
        x: 19,
        width: 5,
        label: "",
        grid_pos: Some((2, 2)),
    },
    PhysicalKey {
        row: 2,
        x: 24,
        width: 5,
        label: "",
        grid_pos: Some((2, 3)),
    },
    PhysicalKey {
        row: 2,
        x: 29,
        width: 5,
        label: "",
        grid_pos: Some((2, 4)),
    },
    PhysicalKey {
        row: 2,
        x: 34,
        width: 5,
        label: "",
        grid_pos: Some((2, 5)),
    },
    PhysicalKey {
        row: 2,
        x: 39,
        width: 5,
        label: "",
        grid_pos: Some((2, 6)),
    },
    PhysicalKey {
        row: 2,
        x: 44,
        width: 5,
        label: "",
        grid_pos: Some((2, 7)),
    },
    PhysicalKey {
        row: 2,
        x: 49,
        width: 5,
        label: "",
        grid_pos: Some((2, 8)),
    },
    PhysicalKey {
        row: 2,
        x: 54,
        width: 5,
        label: "",
        grid_pos: Some((2, 9)),
    },
    PhysicalKey {
        row: 2,
        x: 59,
        width: 5,
        label: "",
        grid_pos: Some((2, 10)),
    },
    PhysicalKey {
        row: 2,
        x: 64,
        width: 11,
        label: "Enter",
        grid_pos: None,
    },
    // Row 3: LShift(11) + 10×1u + RShift(14) = 75
    PhysicalKey {
        row: 3,
        x: 0,
        width: 11,
        label: "Shift",
        grid_pos: None,
    },
    PhysicalKey {
        row: 3,
        x: 11,
        width: 5,
        label: "",
        grid_pos: Some((3, 0)),
    },
    PhysicalKey {
        row: 3,
        x: 16,
        width: 5,
        label: "",
        grid_pos: Some((3, 1)),
    },
    PhysicalKey {
        row: 3,
        x: 21,
        width: 5,
        label: "",
        grid_pos: Some((3, 2)),
    },
    PhysicalKey {
        row: 3,
        x: 26,
        width: 5,
        label: "",
        grid_pos: Some((3, 3)),
    },
    PhysicalKey {
        row: 3,
        x: 31,
        width: 5,
        label: "",
        grid_pos: Some((3, 4)),
    },
    PhysicalKey {
        row: 3,
        x: 36,
        width: 5,
        label: "",
        grid_pos: Some((3, 5)),
    },
    PhysicalKey {
        row: 3,
        x: 41,
        width: 5,
        label: "",
        grid_pos: Some((3, 6)),
    },
    PhysicalKey {
        row: 3,
        x: 46,
        width: 5,
        label: "",
        grid_pos: Some((3, 7)),
    },
    PhysicalKey {
        row: 3,
        x: 51,
        width: 5,
        label: "",
        grid_pos: Some((3, 8)),
    },
    PhysicalKey {
        row: 3,
        x: 56,
        width: 5,
        label: "",
        grid_pos: Some((3, 9)),
    },
    PhysicalKey {
        row: 3,
        x: 61,
        width: 14,
        label: "Shift",
        grid_pos: None,
    },
    // Row 4: Ctrl(8)+Sup(7)+Alt(7)+Space(25)+Alt(7)+Sup(7)+Fn(6)+Ctrl(8) = 75
    PhysicalKey {
        row: 4,
        x: 0,
        width: 8,
        label: "Ctrl",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 8,
        width: 7,
        label: "Sup",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 15,
        width: 7,
        label: "Alt",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 22,
        width: 25,
        label: "",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 47,
        width: 7,
        label: "Alt",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 54,
        width: 7,
        label: "Sup",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 61,
        width: 6,
        label: "Fn",
        grid_pos: None,
    },
    PhysicalKey {
        row: 4,
        x: 67,
        width: 8,
        label: "Ctrl",
        grid_pos: None,
    },
];

pub struct KeyboardWidget<'a> {
    layout: &'a Layout,
    active_keys: &'a [char],
    theme: &'a Theme,
}

impl<'a> KeyboardWidget<'a> {
    pub fn new(layout: &'a Layout, active_keys: &'a [char], theme: &'a Theme) -> Self {
        Self {
            layout,
            active_keys,
            theme,
        }
    }

    pub fn required_height() -> u16 {
        KEY_HEIGHT * 5
    }
}

impl Widget for KeyboardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inactive_text = Style::default().fg(self.theme.keyboard.inactive.text());
        let inactive_border = Style::default().fg(self.theme.keyboard.inactive.border());

        for pk in PHYSICAL_KEYS {
            let x = area.x + pk.x;
            let y = area.y + pk.row as u16 * KEY_HEIGHT;

            if x + pk.width > area.right() || y + KEY_HEIGHT > area.bottom() {
                continue;
            }

            let rect = Rect::new(x, y, pk.width, KEY_HEIGHT);

            if let Some((row, col)) = pk.grid_pos {
                // Character key — look up from layout
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
                        let text_style = Style::default().fg(self.theme.keyboard.active.text());
                        let border_style = Style::default().fg(self.theme.keyboard.active.border());
                        draw_key_box(buf, rect, border_style);
                        draw_key_label(buf, rect, label, text_style);
                    } else {
                        draw_key_box(buf, rect, inactive_border);
                        draw_key_label(buf, rect, label, inactive_text);
                    }
                } else {
                    // Physical position exists but no layout key defined
                    draw_key_box(buf, rect, inactive_border);
                    draw_key_label(buf, rect, "", inactive_text);
                }
            } else {
                // Modifier/decorative key
                draw_key_box(buf, rect, inactive_border);
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

fn draw_key_box(buf: &mut Buffer, rect: Rect, style: Style) {
    let buf_right = buf.area().right();
    let buf_bottom = buf.area().bottom();

    // Top border
    if rect.y < buf_bottom {
        for dx in 1..rect.width.saturating_sub(1) {
            let x = rect.x + dx;
            if x < buf_right {
                buf[(x, rect.y)].set_char('─').set_style(style);
            }
        }
    }

    // Bottom border
    let bot = rect.y + rect.height - 1;
    if bot < buf_bottom {
        for dx in 1..rect.width.saturating_sub(1) {
            let x = rect.x + dx;
            if x < buf_right {
                buf[(x, bot)].set_char('─').set_style(style);
            }
        }
    }

    // Side borders
    for dy in 1..rect.height.saturating_sub(1) {
        let y = rect.y + dy;
        if y < buf_bottom {
            if rect.x < buf_right {
                buf[(rect.x, y)].set_char('│').set_style(style);
            }
            let right = rect.x + rect.width - 1;
            if right < buf_right {
                buf[(right, y)].set_char('│').set_style(style);
            }
        }
    }

    // Corners
    let corners = [
        (rect.x, rect.y, '┌'),
        (rect.x + rect.width - 1, rect.y, '┐'),
        (rect.x, rect.y + rect.height - 1, '└'),
        (rect.x + rect.width - 1, rect.y + rect.height - 1, '┘'),
    ];
    for (x, y, c) in corners {
        if x < buf_right && y < buf_bottom {
            buf[(x, y)].set_char(c).set_style(style);
        }
    }
}
