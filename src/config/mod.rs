pub mod theme;

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

pub use theme::Theme;

use crate::ui::color_cycle;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub theme: Theme,
    pub effect: EffectConfig,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct EffectConfig {
    pub enabled: bool,
    pub cycle_colors: Vec<String>,
}

impl Default for EffectConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cycle_colors: vec![],
        }
    }
}

impl EffectConfig {
    pub fn resolve_palette(&self) -> Vec<(u8, u8, u8)> {
        if self.cycle_colors.is_empty() {
            return color_cycle::expand_palette(&Self::mocha_defaults());
        }

        let mut parsed = Vec::new();
        for entry in &self.cycle_colors {
            let Some(hex) = entry.strip_prefix('#') else {
                tracing::warn!("invalid cycle_colors entry '{}', falling back to default", entry);
                return color_cycle::expand_palette(&Self::mocha_defaults());
            };
            if hex.len() != 6 {
                tracing::warn!("invalid cycle_colors entry '{}', falling back to default", entry);
                return color_cycle::expand_palette(&Self::mocha_defaults());
            }
            match (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                (Ok(r), Ok(g), Ok(b)) => parsed.push((r, g, b)),
                _ => {
                    tracing::warn!("invalid cycle_colors entry '{}', falling back to default", entry);
                    return color_cycle::expand_palette(&Self::mocha_defaults());
                }
            }
        }

        parsed.truncate(8);
        color_cycle::expand_palette(&parsed)
    }

    fn mocha_defaults() -> Vec<(u8, u8, u8)> {
        use catppuccin::PALETTE;
        let c = &PALETTE.mocha.colors;
        vec![
            (c.red.rgb.r, c.red.rgb.g, c.red.rgb.b),
            (c.peach.rgb.r, c.peach.rgb.g, c.peach.rgb.b),
            (c.yellow.rgb.r, c.yellow.rgb.g, c.yellow.rgb.b),
            (c.green.rgb.r, c.green.rgb.g, c.green.rgb.b),
            (c.sky.rgb.r, c.sky.rgb.g, c.sky.rgb.b),
            (c.blue.rgb.r, c.blue.rgb.g, c.blue.rgb.b),
            (c.mauve.rgb.r, c.mauve.rgb.g, c.mauve.rgb.b),
        ]
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct General {
    pub qwerty_remap: bool,
    pub layouts: Vec<String>,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let path = match path {
            Some(p) => {
                if !p.exists() {
                    bail!("Config file not found: {}", p.display());
                }
                Some(p)
            }
            None => default_config_path().filter(|p| p.exists()),
        };

        match path {
            Some(p) => {
                let content = std::fs::read_to_string(&p)
                    .with_context(|| format!("Failed to read config: {}", p.display()))?;
                let config: Config = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config: {}", p.display()))?;
                Ok(config)
            }
            None => Ok(Config::default()),
        }
    }
}

fn default_config_path() -> Option<PathBuf> {
    crate::utils::config_dir().map(|d| d.join("keydrill").join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = Config::default();
        assert!(!config.general.qwerty_remap);
        assert!(config.general.layouts.is_empty());
    }

    #[test]
    fn parse_empty_toml() {
        let config: Config = toml::from_str("").unwrap();
        assert!(!config.general.qwerty_remap);
    }

    #[test]
    fn parse_general_section() {
        let config: Config = toml::from_str(
            r#"
            [general]
            qwerty_remap = true
            layouts = ["/tmp/test.toml"]
            "#,
        )
        .unwrap();
        assert!(config.general.qwerty_remap);
        assert_eq!(config.general.layouts, vec!["/tmp/test.toml"]);
    }

    #[test]
    fn parse_theme_section() {
        let config: Config = toml::from_str(
            r##"
            [theme]
            highlight = "#ff0000"

            [theme.word]
            correct = "cyan"
            "##,
        )
        .unwrap();
        assert_eq!(config.theme.highlight, "#ff0000");
        assert_eq!(config.theme.word.correct, "cyan");
        // Unset fields get defaults
        assert_eq!(config.theme.word.incorrect, "red");
    }

    #[test]
    fn load_nonexistent_explicit_path_errors() {
        let result = Config::load(Some(PathBuf::from("/tmp/keydrill-nonexistent.toml")));
        assert!(result.is_err());
    }
}
