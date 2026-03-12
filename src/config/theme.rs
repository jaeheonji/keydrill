use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct Theme {
    pub text: String,
    pub primary: String,
    pub secondary: String,
    pub highlight: String,
    pub word: WordTheme,
    pub keyboard: KeyboardTheme,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct WordTheme {
    pub current: String,
    pub correct: String,
    pub incorrect: String,
    pub queue: String,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct KeyboardTheme {
    pub active: KeyStyle,
    pub inactive: KeyStyle,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct KeyStyle {
    pub text: String,
    pub border: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text: "reset".into(),
            primary: "green".into(),
            secondary: "dark_gray".into(),
            highlight: "blue".into(),
            word: WordTheme::default(),
            keyboard: KeyboardTheme::default(),
        }
    }
}

impl Default for WordTheme {
    fn default() -> Self {
        Self {
            current: "reset".into(),
            correct: "green".into(),
            incorrect: "red".into(),
            queue: "dark_gray".into(),
        }
    }
}

impl Default for KeyboardTheme {
    fn default() -> Self {
        Self {
            active: KeyStyle {
                text: "white".into(),
                border: "white".into(),
            },
            inactive: KeyStyle {
                text: "dark_gray".into(),
                border: "dark_gray".into(),
            },
        }
    }
}

impl Default for KeyStyle {
    fn default() -> Self {
        Self {
            text: "white".into(),
            border: "white".into(),
        }
    }
}

impl Theme {
    pub fn text(&self) -> Color {
        parse_color(&self.text)
    }

    pub fn primary(&self) -> Color {
        parse_color(&self.primary)
    }

    pub fn secondary(&self) -> Color {
        parse_color(&self.secondary)
    }

    pub fn highlight(&self) -> Color {
        parse_color(&self.highlight)
    }
}

impl WordTheme {
    pub fn current(&self) -> Color {
        parse_color(&self.current)
    }

    pub fn correct(&self) -> Color {
        parse_color(&self.correct)
    }

    pub fn incorrect(&self) -> Color {
        parse_color(&self.incorrect)
    }

    pub fn queue(&self) -> Color {
        parse_color(&self.queue)
    }
}

impl KeyStyle {
    pub fn text(&self) -> Color {
        parse_color(&self.text)
    }

    pub fn border(&self) -> Color {
        parse_color(&self.border)
    }
}

/// Parse a color string into a `ratatui::style::Color`.
///
/// Supported formats:
/// - Named colors: `"black"`, `"red"`, `"green"`, `"yellow"`, `"blue"`,
///   `"magenta"`, `"cyan"`, `"white"`, `"dark_gray"`, `"gray"`, `"reset"`
/// - 256-color index: `"0"` .. `"255"`
/// - Hex RGB: `"#rrggbb"`
fn parse_color(s: &str) -> Color {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#')
        && hex.len() == 6
        && let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        )
    {
        return Color::Rgb(r, g, b);
    }
    match s.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "gray" => Color::Gray,
        "dark_gray" => Color::DarkGray,
        "light_red" => Color::LightRed,
        "light_green" => Color::LightGreen,
        "light_yellow" => Color::LightYellow,
        "light_blue" => Color::LightBlue,
        "light_magenta" => Color::LightMagenta,
        "light_cyan" => Color::LightCyan,
        "reset" => Color::Reset,
        other => match other.parse::<u8>() {
            Ok(n) => Color::Indexed(n),
            Err(_) => Color::Reset,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_named_colors() {
        assert_eq!(parse_color("red"), Color::Red);
        assert_eq!(parse_color("green"), Color::Green);
        assert_eq!(parse_color("dark_gray"), Color::DarkGray);
        assert_eq!(parse_color("light_cyan"), Color::LightCyan);
    }

    #[test]
    fn parse_hex_color() {
        assert_eq!(parse_color("#ff0000"), Color::Rgb(255, 0, 0));
        assert_eq!(parse_color("#00ff00"), Color::Rgb(0, 255, 0));
        assert_eq!(parse_color("#1a2b3c"), Color::Rgb(0x1a, 0x2b, 0x3c));
    }

    #[test]
    fn parse_indexed_color() {
        assert_eq!(parse_color("0"), Color::Indexed(0));
        assert_eq!(parse_color("255"), Color::Indexed(255));
        assert_eq!(parse_color("42"), Color::Indexed(42));
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!(parse_color("RED"), Color::Red);
        assert_eq!(parse_color("Blue"), Color::Blue);
    }

    #[test]
    fn parse_trims_whitespace() {
        assert_eq!(parse_color("  red  "), Color::Red);
        assert_eq!(parse_color(" #ff0000 "), Color::Rgb(255, 0, 0));
    }

    #[test]
    fn parse_invalid_returns_reset() {
        assert_eq!(parse_color("notacolor"), Color::Reset);
        assert_eq!(parse_color("#zzzzzz"), Color::Reset);
        assert_eq!(parse_color("#fff"), Color::Reset); // too short
    }

    #[test]
    fn default_theme_parses_all_fields() {
        let theme = Theme::default();
        assert_eq!(theme.text(), Color::Reset);
        assert_ne!(theme.primary(), Color::Reset);
        assert_ne!(theme.secondary(), Color::Reset);
        assert_ne!(theme.highlight(), Color::Reset);
        assert_ne!(theme.word.correct(), Color::Reset);
        assert_ne!(theme.word.incorrect(), Color::Reset);
        assert_eq!(theme.word.current(), Color::Reset);
        assert_ne!(theme.word.queue(), Color::Reset);
        assert_ne!(theme.keyboard.active.border(), Color::Reset);
        assert_ne!(theme.keyboard.inactive.border(), Color::Reset);
    }
}
