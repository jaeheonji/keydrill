use std::collections::HashMap;
use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::config::Config;
use crate::layout::Layout;
use crate::words;

#[derive(Default)]
pub enum Screen {
    #[default]
    LevelSelect,
    Typing,
    Results,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectPhase {
    #[default]
    Layout,
    Level,
}

/// Level select screen state.
pub struct SelectState {
    pub select_phase: SelectPhase,
    pub current_layout_idx: usize,
    pub current_level: usize,
    pub select_screen_entered_at: Instant,
}

impl Default for SelectState {
    fn default() -> Self {
        Self {
            select_phase: SelectPhase::default(),
            current_layout_idx: 0,
            current_level: 0,
            select_screen_entered_at: Instant::now(),
        }
    }
}

#[derive(Default)]
/// Active typing session state.
pub struct TypingState {
    pub current_word: String,
    pub input: String,
    pub word_queue: Vec<String>,
    pub all_words: Vec<String>,
}

#[derive(Default)]
/// Typing metrics, written during typing and read during results.
pub struct Stats {
    pub total_chars: usize,
    pub correct_chars: usize,
    pub total_words: usize,
    pub completed_words: usize,
    pub typing_started_at: Option<Instant>,
    pub elapsed_on_finish: Option<f64>,
}

#[derive(Default)]
/// QWERTY remap state, toggled on any screen and used during typing.
pub struct RemapState {
    pub qwerty_remap: bool,
    pub qwerty_remap_table: HashMap<char, char>,
}

#[derive(Default)]
pub struct App {
    pub screen: Screen,
    pub layouts: Vec<Layout>,
    pub should_quit: bool,
    pub select: SelectState,
    pub typing: TypingState,
    pub stats: Stats,
    pub remap: RemapState,
}

impl App {
    pub fn new(layouts: Vec<Layout>, config: &Config) -> Self {
        let qwerty_remap_table = layouts[0].build_qwerty_remap();
        Self {
            layouts,
            remap: RemapState {
                qwerty_remap: config.general.qwerty_remap,
                qwerty_remap_table,
            },
            ..Self::default()
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layouts[self.select.current_layout_idx]
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<char> {
        match self.screen {
            Screen::LevelSelect => {
                self.handle_level_select_key(key);
                None
            }
            Screen::Typing => self.handle_typing_key(key),
            Screen::Results => {
                self.handle_results_key(key);
                None
            }
        }
    }

    fn handle_level_select_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('t') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.remap.qwerty_remap = !self.remap.qwerty_remap;
            return;
        }
        match self.select.select_phase {
            SelectPhase::Layout => self.handle_layout_phase_key(key),
            SelectPhase::Level => self.handle_level_phase_key(key),
        }
    }

    fn handle_layout_phase_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Up => {
                if self.select.current_layout_idx > 0 {
                    self.select.current_layout_idx -= 1;
                }
            }
            KeyCode::Down => {
                if self.select.current_layout_idx + 1 < self.layouts.len() {
                    self.select.current_layout_idx += 1;
                }
            }
            KeyCode::Enter | KeyCode::Right => {
                self.remap.qwerty_remap_table = self.layout().build_qwerty_remap();
                self.select.current_level = 0;
                self.select.select_phase = SelectPhase::Level;
            }
            _ => {}
        }
    }

    fn handle_level_phase_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Left => {
                if self.layouts.len() > 1 {
                    self.select.select_phase = SelectPhase::Layout;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Up => {
                if self.select.current_level > 0 {
                    self.select.current_level -= 1;
                }
            }
            KeyCode::Down => {
                if self.select.current_level + 1 < self.layout().levels.len() {
                    self.select.current_level += 1;
                }
            }
            KeyCode::Enter | KeyCode::Right => {
                self.start_typing();
            }
            _ => {}
        }
    }

    fn handle_typing_key(&mut self, key: KeyEvent) -> Option<char> {
        if key.code == KeyCode::Char('t') && key.modifiers.contains(KeyModifiers::CONTROL) {
            tracing::debug!("toggled qwerty remap: {}", !self.remap.qwerty_remap);
            self.remap.qwerty_remap = !self.remap.qwerty_remap;
            return None;
        }
        match key.code {
            KeyCode::Esc => {
                tracing::debug!("esc");
                self.screen = Screen::LevelSelect;
                self.select.select_screen_entered_at = Instant::now();
                self.typing.input.clear();
                None
            }
            KeyCode::Enter => {
                if !self.typing.input.is_empty() {
                    self.score_word();
                    self.advance_word();
                }
                None
            }
            KeyCode::Char(c) => {
                if self.stats.typing_started_at.is_none() {
                    self.stats.typing_started_at = Some(Instant::now());
                }
                let c = if self.remap.qwerty_remap {
                    self.remap.qwerty_remap_table.get(&c).copied().unwrap_or(c)
                } else {
                    c
                };
                let expected = self
                    .typing
                    .current_word
                    .chars()
                    .nth(self.typing.input.len());
                let correct = expected == Some(c);
                tracing::debug!(typed = %c, ?expected, correct, word = %self.typing.current_word, "keypress");
                self.typing.input.push(c);
                Some(c)
            }
            KeyCode::Backspace => {
                tracing::debug!("backspace");
                self.typing.input.pop();
                None
            }
            _ => None,
        }
    }

    fn start_typing(&mut self) {
        self.screen = Screen::Typing;
        self.typing.input.clear();
        self.typing.word_queue.clear();
        self.stats = Stats::default();
        self.load_words();
        self.typing.word_queue =
            words::shuffled_batch(&self.typing.all_words, self.typing.all_words.len());
        self.stats.total_words = self.typing.word_queue.len();
        self.typing.current_word = self.typing.word_queue.remove(0);
    }

    fn load_words(&mut self) {
        let layout = &self.layouts[self.select.current_layout_idx];
        let keys = layout.available_keys_for_level(self.select.current_level);
        self.typing.all_words =
            words::build_word_pool(&layout.levels, self.select.current_level, &keys);
    }

    fn score_word(&mut self) {
        let word_len = self.typing.current_word.chars().count();
        self.stats.total_chars += word_len;
        for (typed, expected) in self
            .typing
            .input
            .chars()
            .zip(self.typing.current_word.chars())
        {
            if typed == expected {
                self.stats.correct_chars += 1;
            }
        }
        self.stats.completed_words += 1;
    }

    fn advance_word(&mut self) {
        if self.typing.word_queue.is_empty() {
            // Session complete
            self.stats.elapsed_on_finish = self
                .stats
                .typing_started_at
                .map(|t| t.elapsed().as_secs_f64());
            self.screen = Screen::Results;
            self.typing.input.clear();
            return;
        }
        self.typing.current_word = self.typing.word_queue.remove(0);
        self.typing.input.clear();
    }

    fn handle_results_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => self.start_typing(),
            KeyCode::Esc => {
                self.screen = Screen::LevelSelect;
                self.select.select_screen_entered_at = Instant::now();
            }
            _ => {}
        }
    }

    pub fn available_keys(&self) -> Vec<char> {
        self.layout()
            .available_keys_for_level(self.select.current_level)
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.stats
            .elapsed_on_finish
            .or_else(|| {
                self.stats
                    .typing_started_at
                    .map(|t| t.elapsed().as_secs_f64())
            })
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEventKind;

    fn test_app() -> App {
        let layouts = crate::layout::Layout::discover_all(&[]);
        let config = Config::default();
        App::new(layouts, &config)
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new_with_kind(code, KeyModifiers::NONE, KeyEventKind::Press)
    }

    fn ctrl_key(c: char) -> KeyEvent {
        KeyEvent::new_with_kind(KeyCode::Char(c), KeyModifiers::CONTROL, KeyEventKind::Press)
    }

    // -- Level select navigation --

    #[test]
    fn initial_screen_is_level_select() {
        let app = test_app();
        assert!(matches!(app.screen, Screen::LevelSelect));
        assert_eq!(app.select.select_phase, SelectPhase::Layout);
    }

    #[test]
    fn enter_selects_layout_and_moves_to_level_phase() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter));
        assert_eq!(app.select.select_phase, SelectPhase::Level);
    }

    #[test]
    fn esc_on_layout_phase_quits() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Esc));
        assert!(app.should_quit);
    }

    #[test]
    fn esc_on_level_phase_with_single_layout_quits() {
        let mut app = test_app();
        // Force single layout
        app.layouts.truncate(1);
        app.select.select_phase = SelectPhase::Level;
        app.handle_key(key(KeyCode::Esc));
        assert!(app.should_quit);
    }

    #[test]
    fn esc_on_level_phase_with_multiple_layouts_goes_to_layout_phase() {
        let mut app = test_app();
        // Ensure multiple layouts
        let extra = app.layouts[0].clone();
        app.layouts.push(extra);
        app.select.select_phase = SelectPhase::Level;
        app.handle_key(key(KeyCode::Esc));
        assert!(!app.should_quit);
        assert_eq!(app.select.select_phase, SelectPhase::Layout);
    }

    #[test]
    fn level_navigation_clamps() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter)); // → level phase
        // Already at 0, up shouldn't underflow
        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.select.current_level, 0);
        // Move down then back up
        app.handle_key(key(KeyCode::Down));
        assert_eq!(app.select.current_level, 1);
        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.select.current_level, 0);
    }

    // -- Typing --

    #[test]
    fn enter_on_level_starts_typing() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter)); // → level phase
        app.handle_key(key(KeyCode::Enter)); // → typing
        assert!(matches!(app.screen, Screen::Typing));
        assert!(!app.typing.current_word.is_empty());
    }

    #[test]
    fn typing_correct_chars_and_enter_advances() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Enter));

        let word = app.typing.current_word.clone();
        // Type the full word correctly
        for ch in word.chars() {
            app.handle_key(key(KeyCode::Char(ch)));
        }
        // Should NOT have advanced yet (no Enter)
        assert_eq!(app.typing.current_word, word);
        // Press Enter to submit
        app.handle_key(key(KeyCode::Enter));
        // Should have advanced to next word or results
        assert_ne!(app.typing.current_word, word);
    }

    #[test]
    fn typing_without_enter_does_not_advance() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Enter));

        let word = app.typing.current_word.clone();
        for ch in word.chars() {
            app.handle_key(key(KeyCode::Char(ch)));
        }
        assert_eq!(app.typing.current_word, word);
    }

    #[test]
    fn backspace_removes_input() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Enter));

        app.handle_key(key(KeyCode::Char('x')));
        assert_eq!(app.typing.input.len(), 1);
        app.handle_key(key(KeyCode::Backspace));
        assert!(app.typing.input.is_empty());
    }

    #[test]
    fn esc_during_typing_returns_to_level_select() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Esc));
        assert!(matches!(app.screen, Screen::LevelSelect));
    }

    // -- Scoring --

    #[test]
    fn score_word_counts_correctly() {
        let mut app = test_app();
        app.typing.current_word = "abc".into();
        app.typing.input = "abc".into();
        app.score_word();
        assert_eq!(app.stats.total_chars, 3);
        assert_eq!(app.stats.correct_chars, 3);
        assert_eq!(app.stats.completed_words, 1);
    }

    #[test]
    fn score_word_counts_mismatches() {
        let mut app = test_app();
        app.typing.current_word = "abc".into();
        app.typing.input = "axc".into();
        app.score_word();
        assert_eq!(app.stats.total_chars, 3);
        assert_eq!(app.stats.correct_chars, 2);
    }

    // -- Remap toggle --

    #[test]
    fn ctrl_t_toggles_remap() {
        let mut app = test_app();
        let initial = app.remap.qwerty_remap;
        app.handle_key(ctrl_key('t'));
        assert_ne!(app.remap.qwerty_remap, initial);
        app.handle_key(ctrl_key('t'));
        assert_eq!(app.remap.qwerty_remap, initial);
    }

    // -- Results --

    #[test]
    fn results_esc_returns_to_level_select() {
        let mut app = test_app();
        app.screen = Screen::Results;
        app.handle_key(key(KeyCode::Esc));
        assert!(matches!(app.screen, Screen::LevelSelect));
    }

    #[test]
    fn results_enter_restarts_typing() {
        let mut app = test_app();
        app.handle_key(key(KeyCode::Enter)); // → level phase
        app.handle_key(key(KeyCode::Enter)); // → typing (loads words)
        app.screen = Screen::Results;
        app.handle_key(key(KeyCode::Enter));
        assert!(matches!(app.screen, Screen::Typing));
    }
}
