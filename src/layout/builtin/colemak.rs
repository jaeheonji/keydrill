use crate::layout::types::{KeyMapping, KeyPosition, Layout, Level};

pub fn builtin_colemak() -> Layout {
    Layout {
        name: "Colemak".into(),
        builtin: None,
        keys: colemak_keys(),
        levels: colemak_levels(),
    }
}

fn colemak_keys() -> Vec<KeyMapping> {
    let rows: &[&[char]] = &[
        // Row 0: number row
        &[
            '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
        ],
        // Row 1: top letter row (Colemak)
        &['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';'],
        // Row 2: home row (Colemak)
        &['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o', '\''],
        // Row 3: bottom row (Colemak)
        &['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
    ];

    let mut keys = Vec::new();
    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            keys.push(KeyMapping {
                position: KeyPosition {
                    row: row_idx as u8,
                    col: col_idx as u8,
                },
                normal: ch,
            });
        }
    }
    keys
}

fn colemak_levels() -> Vec<Level> {
    vec![
        Level {
            name: "Home Row (Left)".into(),
            new_keys: vec!['a', 'r', 's', 't'],
            words: vec![
                "art", "rat", "star", "tart", "start", "sat", "tar", "at", "as", "a",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Home Row (Right)".into(),
            new_keys: vec!['n', 'e', 'i', 'o'],
            words: vec![
                "ten", "net", "set", "rest", "nest", "test", "sent", "rent", "in", "it", "its",
                "sit", "sin", "tin", "nine", "tie", "tire", "site", "stein", "inert", "stern",
                "insert", "risen",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Home Row (Full)".into(),
            new_keys: vec!['d', 'h'],
            words: vec![
                "the", "that", "this", "then", "than", "their", "there", "these", "third", "those",
                "thin", "had", "hand", "hard", "head", "heat", "hint", "hit", "shed", "she",
                "shine", "shirt", "dish", "his",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Top Row (Left)".into(),
            new_keys: vec!['q', 'w', 'f', 'p', 'g'],
            words: vec![
                "want", "water", "wait", "was", "war", "wash", "what", "white", "wide", "wife",
                "win", "with", "winter", "wish", "wire", "wise", "fit", "first", "find", "fire",
                "fast", "far", "few", "free", "fresh", "pant", "part", "past", "path", "pin",
                "pint", "print", "swift", "drift", "swing", "spring", "gift", "get", "great",
                "grip", "grit",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Top Row (Right)".into(),
            new_keys: vec!['j', 'l', 'u', 'y'],
            words: vec![
                "just", "jet", "jut", "jungle", "judge", "june", "lift", "light", "line", "list",
                "little", "lung", "lust", "until", "useful", "unit", "unique", "upper", "usual",
                "yes", "yet", "yield",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Bottom Row (Left)".into(),
            new_keys: vec!['z', 'x', 'c', 'v', 'b'],
            words: vec![
                "size", "zero", "zone", "zinc", "zest", "next", "exist", "extra", "exact",
                "except", "exit", "context", "citizen", "can", "cut", "cup", "cause", "cent",
                "center", "certain", "chance", "change", "charge", "circle", "van", "vast",
                "visit", "vital", "give", "live", "ever", "even", "seven", "view", "value",
                "voice", "bus", "but", "best", "bit", "big", "bind", "built", "burn",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Bottom Row (Right)".into(),
            new_keys: vec!['k', 'm'],
            words: vec![
                "king", "kind", "keep", "knee", "kill", "kit", "kitchen", "kick", "mark", "make",
                "man", "map", "match", "mean", "meet", "milk", "mind", "mine", "miss", "mix",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Punctuation".into(),
            new_keys: vec![',', '.', '/', ';', '\''],
            words: vec![
                "it's", "that's", "there's", "what's", "can't", "didn't", "don't", "isn't",
                "hasn't", "won't",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_keys_and_levels() {
        let layout = builtin_colemak();
        assert_eq!(layout.name, "Colemak");
        assert!(!layout.keys.is_empty());
        assert!(!layout.levels.is_empty());
    }

    #[test]
    fn test_home_row() {
        let layout = builtin_colemak();
        let home: Vec<char> = layout
            .keys
            .iter()
            .filter(|k| k.position.row == 2)
            .map(|k| k.normal)
            .collect();
        assert_eq!(
            home,
            vec!['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o', '\'']
        );
    }
}
