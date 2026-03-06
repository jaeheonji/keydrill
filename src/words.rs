use rand::RngExt;
use rand::seq::{IndexedRandom, SliceRandom};

use crate::layout::Level;

/// Build the word pool for a given level.
///
/// - `random_words=true` (or no words): current level words + random fill up to `word_count`
/// - `random_words=false` + words: cumulative words from levels 0..=level.
///   If the total exceeds `word_count`, samples down with at least `word_count/2`
///   guaranteed from the current level.
pub fn build_word_pool(levels: &[Level], level: usize, available_keys: &[char]) -> Vec<String> {
    let word_count = levels[level].word_count;
    let random_words = levels[level].random_words;
    let current_words = &levels[level].words;
    let mut rng = rand::rng();

    if random_words || current_words.is_empty() {
        let mut words = current_words.clone();
        if words.len() > word_count {
            words.shuffle(&mut rng);
            words.truncate(word_count);
        } else if words.len() < word_count {
            let remaining = word_count - words.len();
            for _ in 0..remaining {
                let len = rng.random_range(3..=5);
                let word: String = (0..len)
                    .map(|_| *available_keys.choose(&mut rng).unwrap_or(&'a'))
                    .collect();
                words.push(word);
            }
        }
        return words;
    }

    // random_words=false: cumulative words from all levels up to current
    let all_words: Vec<String> = levels[..=level]
        .iter()
        .flat_map(|l| l.words.iter().cloned())
        .collect();

    if all_words.len() <= word_count {
        return all_words;
    }

    // Sample word_count, guaranteeing at least word_count/2 from current level
    let min_current = (word_count / 2).min(current_words.len());

    let mut current_shuffled = current_words.clone();
    current_shuffled.shuffle(&mut rng);
    let guaranteed: Vec<String> = current_shuffled[..min_current].to_vec();
    let rest_current = &current_shuffled[min_current..];

    let prev_words: Vec<String> = levels[..level]
        .iter()
        .flat_map(|l| l.words.iter().cloned())
        .collect();

    let mut candidates: Vec<String> = prev_words;
    candidates.extend_from_slice(rest_current);
    candidates.shuffle(&mut rng);
    candidates.truncate(word_count - min_current);

    let mut pool = guaranteed;
    pool.extend(candidates);
    pool
}

/// Shuffle and return a batch of words.
pub fn shuffled_batch(words: &[String], count: usize) -> Vec<String> {
    let mut rng = rand::rng();
    let mut pool = words.to_vec();
    pool.shuffle(&mut rng);
    pool.into_iter().take(count).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::builtin_colemak;

    #[test]
    fn test_build_word_pool_uses_level_words() {
        let layout = builtin_colemak();
        let keys = layout.available_keys_for_level(0);
        let pool = build_word_pool(&layout.levels, 0, &keys);
        assert!(pool.iter().any(|w| w == "art"));
    }

    #[test]
    fn test_build_word_pool_cumulative() {
        let layout = builtin_colemak();
        let keys = layout.available_keys_for_level(1);
        let pool = build_word_pool(&layout.levels, 1, &keys);
        assert!(!pool.iter().any(|w| w == "art")); // level 0 words excluded
        assert!(pool.iter().any(|w| w == "ten")); // level 1
    }

    #[test]
    fn test_build_word_pool_fills_to_word_count() {
        let layout = builtin_colemak();
        let keys = layout.available_keys_for_level(0);
        let pool = build_word_pool(&layout.levels, 0, &keys);
        assert_eq!(pool.len(), layout.levels[0].word_count);
    }

    #[test]
    fn test_build_word_pool_truncates_when_words_exceed_word_count() {
        let layout = builtin_colemak();
        // Level 3 (Top Row Left) has 41 words but word_count=30
        let keys = layout.available_keys_for_level(3);
        let pool = build_word_pool(&layout.levels, 3, &keys);
        assert_eq!(pool.len(), layout.levels[3].word_count);
    }

    #[test]
    fn test_no_random_cumulative_under_word_count() {
        // When cumulative words <= word_count, return all of them
        let levels = vec![
            Level {
                name: "L0".into(),
                new_keys: vec!['a', 'b'],
                words: vec!["ab".into(), "ba".into()],
                word_count: 10,
                random_words: false,
            },
            Level {
                name: "L1".into(),
                new_keys: vec!['c'],
                words: vec!["abc".into()],
                word_count: 10,
                random_words: false,
            },
        ];
        let keys = vec!['a', 'b', 'c'];
        let pool = build_word_pool(&levels, 1, &keys);
        // 2 + 1 = 3 words, all included
        assert_eq!(pool.len(), 3);
        assert!(pool.contains(&"ab".into()));
        assert!(pool.contains(&"ba".into()));
        assert!(pool.contains(&"abc".into()));
    }

    #[test]
    fn test_no_random_cumulative_over_word_count() {
        // When cumulative words > word_count, sample with current-level guarantee
        let levels = vec![
            Level {
                name: "L0".into(),
                new_keys: vec!['a', 'b'],
                words: (0..20).map(|i| format!("a{i}")).collect(),
                word_count: 10,
                random_words: false,
            },
            Level {
                name: "L1".into(),
                new_keys: vec!['c'],
                words: (0..20).map(|i| format!("c{i}")).collect(),
                word_count: 10,
                random_words: false,
            },
        ];
        let keys = vec!['a', 'b', 'c'];
        let pool = build_word_pool(&levels, 1, &keys);
        assert_eq!(pool.len(), 10);
        // At least word_count/2 = 5 from current level
        let current_count = pool.iter().filter(|w| w.starts_with('c')).count();
        assert!(
            current_count >= 5,
            "Expected >= 5 current-level words, got {current_count}"
        );
    }

    #[test]
    fn test_all_words_use_only_available_keys() {
        let layout = builtin_colemak();
        for (i, level) in layout.levels.iter().enumerate() {
            let keys = layout.available_keys_for_level(i);
            for word in &level.words {
                for c in word.chars() {
                    assert!(
                        keys.contains(&c),
                        "Level {i} ({}) word \"{word}\" contains '{c}' not in available keys {keys:?}",
                        level.name,
                    );
                }
            }
        }
    }

    #[test]
    fn test_shuffled_batch() {
        let words = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
        let batch = shuffled_batch(&words, 3);
        assert_eq!(batch.len(), 3);
    }
}
