use rand::RngExt;
use rand::seq::{IndexedRandom, SliceRandom};

use crate::layout::Level;

/// Build the word pool for a given level.
///
/// Collects words cumulatively from all levels up to `level`, then fills
/// remaining slots (up to the current level's `word_count`) with randomly
/// generated words using the available keys.
pub fn build_word_pool(levels: &[Level], level: usize, available_keys: &[char]) -> Vec<String> {
    let word_count = levels[level].word_count;
    let random_words = levels[level].random_words;

    let mut words: Vec<String> = levels[..=level]
        .iter()
        .flat_map(|l| l.words.iter().cloned())
        .collect();

    // random_words=false + words exist → use words only (ignore word_count)
    // random_words=false + no words   → ignore false, generate random up to word_count
    // random_words=true               → fill up to word_count with random words
    let should_generate = random_words || words.is_empty();

    if should_generate && words.len() < word_count {
        let mut rng = rand::rng();
        let remaining = word_count - words.len();
        for _ in 0..remaining {
            let len = rng.random_range(3..=5);
            let word: String = (0..len)
                .map(|_| *available_keys.choose(&mut rng).unwrap_or(&'a'))
                .collect();
            words.push(word);
        }
    }

    words
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
        assert!(pool.iter().any(|w| w == "art")); // level 0
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
    fn test_shuffled_batch() {
        let words = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
        let batch = shuffled_batch(&words, 3);
        assert_eq!(batch.len(), 3);
    }
}
