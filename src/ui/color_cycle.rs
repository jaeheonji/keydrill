use std::time::Duration;

use ratatui::style::Color;

const CYCLE_DURATION_SECS: f32 = 8.0;

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t) as u8
}

const BORDER_ROTATION_SECS: f32 = 2.0;

pub fn border_color(elapsed: Duration, position: f32, colors: &[(u8, u8, u8)]) -> Color {
    let n = colors.len();
    if n == 0 {
        return Color::White;
    }
    let segments = n as f32;
    let phase = (elapsed.as_secs_f32() / BORDER_ROTATION_SECS - position).rem_euclid(1.0);
    let pos = phase * segments;
    let idx = pos as usize % n;
    let next = (idx + 1) % n;
    let frac = pos - pos.floor();
    let (r1, g1, b1) = colors[idx];
    let (r2, g2, b2) = colors[next];
    Color::Rgb(
        lerp_u8(r1, r2, frac),
        lerp_u8(g1, g2, frac),
        lerp_u8(b1, b2, frac),
    )
}

pub fn cycling_color(elapsed: Duration, colors: &[(u8, u8, u8)]) -> Color {
    let n = colors.len();
    if n == 0 {
        return Color::White;
    }
    if n == 1 {
        let (r, g, b) = colors[0];
        return Color::Rgb(r, g, b);
    }
    let segments = (n - 1) as f32;
    let half = CYCLE_DURATION_SECS / 2.0;
    let t = elapsed.as_secs_f32() % CYCLE_DURATION_SECS;
    let progress = if t < half {
        t / half
    } else {
        1.0 - (t - half) / half
    };
    let pos = progress * segments;
    let idx = (pos as usize).min(n - 2);
    let frac = pos - idx as f32;
    let (r1, g1, b1) = colors[idx];
    let (r2, g2, b2) = colors[idx + 1];
    Color::Rgb(
        lerp_u8(r1, r2, frac),
        lerp_u8(g1, g2, frac),
        lerp_u8(b1, b2, frac),
    )
}

/// Expand an input palette to exactly 8 colors.
/// Distributes input colors evenly around the 8 slots, fills gaps by HSL interpolation.
/// For 1 color: generates 8 lightness steps varying L ±0.15 around base.
pub fn expand_palette(input: &[(u8, u8, u8)]) -> Vec<(u8, u8, u8)> {
    if input.is_empty() {
        return vec![(255, 255, 255); 8];
    }
    if input.len() >= 8 {
        return input[..8].to_vec();
    }

    let n = input.len();

    if n == 1 {
        let (h, s, l) = rgb_to_hsl(input[0].0, input[0].1, input[0].2);
        return (0..8)
            .map(|i| {
                let offset = -0.15 + (0.30 * i as f64 / 7.0);
                let new_l = (l + offset).clamp(0.05, 0.95);
                hsl_to_rgb(h, s, new_l)
            })
            .collect();
    }

    // Place n colors evenly across 8 slots, interpolate gaps
    let mut result = [(0u8, 0u8, 0u8); 8];
    // Positions of input colors in the 8-slot ring
    let positions: Vec<f64> = (0..n).map(|i| 8.0 * i as f64 / n as f64).collect();

    for slot in 0..8 {
        let slot_f = slot as f64;
        // Find which two input colors this slot falls between
        let mut before_idx = 0;
        for (i, &pos) in positions.iter().enumerate() {
            if pos <= slot_f {
                before_idx = i;
            }
        }
        let after_idx = (before_idx + 1) % n;

        let before_pos = positions[before_idx];
        let after_pos = if after_idx == 0 {
            positions[0] + 8.0
        } else {
            positions[after_idx]
        };

        let span = after_pos - before_pos;
        if span < 1e-6 {
            result[slot] = input[before_idx];
            continue;
        }

        let t = ((slot_f - before_pos).rem_euclid(8.0)) / span;
        let t = t.clamp(0.0, 1.0);

        let (h1, s1, l1) = rgb_to_hsl(input[before_idx].0, input[before_idx].1, input[before_idx].2);
        let (h2, s2, l2) = rgb_to_hsl(input[after_idx].0, input[after_idx].1, input[after_idx].2);

        // Shortest-path hue interpolation
        let mut dh = h2 - h1;
        if dh > 180.0 {
            dh -= 360.0;
        } else if dh < -180.0 {
            dh += 360.0;
        }
        let h = (h1 + dh * t).rem_euclid(360.0);
        let s = s1 + (s2 - s1) * t;
        let l = l1 + (l2 - l1) * t;

        result[slot] = hsl_to_rgb(h, s, l);
    }

    result.to_vec()
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < 1e-10 {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < 1e-10 {
        let mut h = (g - b) / d;
        if g < b {
            h += 6.0;
        }
        h
    } else if (max - g).abs() < 1e-10 {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    (h * 60.0, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    if s.abs() < 1e-10 {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h = h / 360.0;

    let hue_to_rgb = |p: f64, q: f64, mut t: f64| -> f64 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0 / 2.0 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };

    let r = (hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0).round() as u8;
    let g = (hue_to_rgb(p, q, h) * 255.0).round() as u8;
    let b = (hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0).round() as u8;
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_single_color() {
        let result = expand_palette(&[(255, 0, 0)]);
        assert_eq!(result.len(), 8);
        // All should share the same hue
    }

    #[test]
    fn expand_two_colors() {
        let result = expand_palette(&[(255, 0, 0), (0, 0, 255)]);
        assert_eq!(result.len(), 8);
        // First slot should be close to red
        assert_eq!(result[0], (255, 0, 0));
    }

    #[test]
    fn expand_eight_colors_passthrough() {
        let input: Vec<(u8, u8, u8)> = (0..8).map(|i| (i * 30, 100, 200)).collect();
        let result = expand_palette(&input);
        assert_eq!(result, input);
    }

    #[test]
    fn expand_more_than_eight_truncates() {
        let input: Vec<(u8, u8, u8)> = (0..10).map(|i| (i * 20, 50, 150)).collect();
        let result = expand_palette(&input);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn hsl_roundtrip() {
        let colors = [(255, 0, 0), (0, 255, 0), (0, 0, 255), (128, 64, 192)];
        for (r, g, b) in colors {
            let (h, s, l) = rgb_to_hsl(r, g, b);
            let (r2, g2, b2) = hsl_to_rgb(h, s, l);
            assert!((r as i16 - r2 as i16).unsigned_abs() <= 1, "r: {r} vs {r2}");
            assert!((g as i16 - g2 as i16).unsigned_abs() <= 1, "g: {g} vs {g2}");
            assert!((b as i16 - b2 as i16).unsigned_abs() <= 1, "b: {b} vs {b2}");
        }
    }
}
