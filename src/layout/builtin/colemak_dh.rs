use crate::layout::types::{KeyMapping, KeyPosition, Layout, Level};

pub fn builtin_colemak_dh() -> Layout {
    Layout {
        name: "Colemak-DH".into(),
        builtin: None,
        keys: colemak_dh_keys(),
        levels: colemak_dh_levels(),
    }
}

fn colemak_dh_keys() -> Vec<KeyMapping> {
    let rows: &[&[char]] = &[
        // Row 0: number row
        &[
            '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
        ],
        // Row 1: top letter row (Colemak-DH)
        &['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';'],
        // Row 2: home row (Colemak-DH)
        &['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o', '\''],
        // Row 3: bottom row (Colemak-DH)
        &['z', 'x', 'c', 'd', 'v', 'k', 'h', ',', '.', '/'],
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

fn colemak_dh_levels() -> Vec<Level> {
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
            new_keys: vec!['g', 'm'],
            words: vec![
                "game", "gate", "gain", "get", "go", "gone", "got", "grant", "great", "grit",
                "gross", "gist", "grime", "grin", "most", "more", "main", "mass", "mine", "miss",
                "mist", "mart", "mat", "mean", "meet", "met", "men", "stem", "term", "team",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Top Row (Left)".into(),
            new_keys: vec!['q', 'w', 'f', 'p', 'b'],
            words: vec![
                "want", "water", "wait", "was", "war", "wash", "what", "white", "wide", "wife",
                "win", "with", "winter", "wish", "wire", "wise", "fit", "first", "find", "fire",
                "fast", "far", "few", "free", "fresh", "pant", "part", "past", "path", "pin",
                "pint", "print", "swift", "bring", "best", "bit", "big", "born", "burn", "brain",
                "bright",
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
            new_keys: vec!['z', 'x', 'c', 'd', 'v'],
            words: vec![
                "size", "zero", "zone", "zinc", "zest", "next", "exist", "extra", "exact",
                "except", "exit", "context", "citizen", "can", "cut", "cup", "cause", "cent",
                "center", "certain", "chance", "change", "charge", "circle", "van", "vast",
                "visit", "vital", "give", "live", "ever", "even", "seven", "view", "value",
                "voice", "did", "down", "draw", "drive", "driven", "during", "dust", "dine",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            word_count: 30,
            random_words: true,
        },
        Level {
            name: "Bottom Row (Right)".into(),
            new_keys: vec!['k', 'h'],
            words: vec![
                "king", "kind", "keep", "knee", "kill", "kit", "kitchen", "kick", "hand", "hard",
                "head", "heat", "hint", "hit", "his", "home", "horse", "house", "human", "hunt",
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
        let layout = builtin_colemak_dh();
        assert_eq!(layout.name, "Colemak-DH");
        assert!(!layout.keys.is_empty());
        assert!(!layout.levels.is_empty());
    }

    #[test]
    fn test_home_row() {
        let layout = builtin_colemak_dh();
        let home: Vec<char> = layout
            .keys
            .iter()
            .filter(|k| k.position.row == 2)
            .map(|k| k.normal)
            .collect();
        assert_eq!(
            home,
            vec!['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o', '\'']
        );
    }
}
