use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct KeyPosition {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyMapping {
    #[serde(flatten)]
    pub position: KeyPosition,
    pub normal: char,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Level {
    pub name: String,
    pub new_keys: Vec<char>,
    #[serde(default)]
    pub words: Vec<String>,
    #[serde(default = "default_word_count")]
    pub word_count: usize,
    #[serde(default = "default_random_words")]
    pub random_words: bool,
}

fn default_word_count() -> usize {
    50
}

fn default_random_words() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct Layout {
    pub name: String,
    pub keys: Vec<KeyMapping>,
    pub levels: Vec<Level>,
}

impl Layout {
    /// Returns all keys available up to and including the given level.
    pub fn available_keys_for_level(&self, level: usize) -> Vec<char> {
        self.levels[..=level]
            .iter()
            .flat_map(|l| l.new_keys.iter().copied())
            .collect()
    }

    /// Build a remap table from QWERTY physical positions to this layout's characters.
    /// For each (row, col) position, maps the QWERTY character to this layout's character.
    /// Identical mappings are excluded.
    pub fn build_qwerty_remap(&self) -> HashMap<char, char> {
        let qwerty_rows: &[&[char]] = &[
            &[
                '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
            ],
            &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\''],
            &['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
        ];

        let mut pos_to_target: HashMap<(u8, u8), char> = HashMap::new();
        for key in &self.keys {
            pos_to_target.insert((key.position.row, key.position.col), key.normal);
        }

        let mut remap = HashMap::new();
        for (row_idx, row) in qwerty_rows.iter().enumerate() {
            for (col_idx, &qwerty_ch) in row.iter().enumerate() {
                if let Some(&target_ch) = pos_to_target.get(&(row_idx as u8, col_idx as u8))
                    && qwerty_ch != target_ch
                {
                    remap.insert(qwerty_ch, target_ch);
                }
            }
        }
        remap
    }
}

#[cfg(test)]
mod tests {
    use super::super::builtin_colemak;

    #[test]
    fn test_available_keys_for_level() {
        let layout = builtin_colemak();
        let keys = layout.available_keys_for_level(0);
        assert_eq!(keys, vec!['a', 'r', 's', 't']);

        let keys = layout.available_keys_for_level(1);
        assert_eq!(keys, vec!['a', 'r', 's', 't', 'n', 'e', 'i', 'o']);
    }
}
