use std::collections::HashMap;
use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::layout::Layout;
use crate::words;

#[derive(Default)]
pub enum Screen {
    #[default]
    LevelSelect,
    Typing,
    Results,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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

pub struct App {
    pub screen: Screen,
    pub layouts: Vec<Layout>,
    pub should_quit: bool,
    pub start_time: Instant,
    pub select: SelectState,
    pub typing: TypingState,
    pub stats: Stats,
    pub remap: RemapState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::default(),
            layouts: Vec::new(),
            should_quit: false,
            start_time: Instant::now(),
            select: SelectState::default(),
            typing: TypingState::default(),
            stats: Stats::default(),
            remap: RemapState::default(),
        }
    }
}

impl App {
    pub fn new(layouts: Vec<Layout>) -> Self {
        let qwerty_remap_table = layouts[0].build_qwerty_remap();
        Self {
            layouts,
            remap: RemapState {
                qwerty_remap_table,
                ..RemapState::default()
            },
            ..Self::default()
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layouts[self.select.current_layout_idx]
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.screen {
            Screen::LevelSelect => self.handle_level_select_key(key),
            Screen::Typing => self.handle_typing_key(key),
            Screen::Results => self.handle_results_key(key),
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

    fn handle_typing_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('t') && key.modifiers.contains(KeyModifiers::CONTROL) {
            tracing::debug!("toggled qwerty remap: {}", !self.remap.qwerty_remap);
            self.remap.qwerty_remap = !self.remap.qwerty_remap;
            return;
        }
        match key.code {
            KeyCode::Esc => {
                tracing::debug!("esc");
                self.screen = Screen::LevelSelect;
                self.select.select_screen_entered_at = Instant::now();
                self.typing.input.clear();
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
                if self.typing.input.len() >= self.typing.current_word.len() {
                    self.score_word();
                    self.advance_word();
                }
            }
            KeyCode::Backspace => {
                tracing::debug!("backspace");
                self.typing.input.pop();
            }
            _ => {}
        }
    }

    fn start_typing(&mut self) {
        self.screen = Screen::Typing;
        self.typing.input.clear();
        self.typing.word_queue.clear();
        self.stats.typing_started_at = None;
        self.stats.total_chars = 0;
        self.stats.correct_chars = 0;
        self.stats.completed_words = 0;
        self.stats.elapsed_on_finish = None;
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
        let word_len = self.typing.current_word.len();
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

    pub fn next_expected_char(&self) -> Option<char> {
        self.typing
            .current_word
            .chars()
            .nth(self.typing.input.len())
    }

    pub fn elapsed_secs(&self) -> f64 {
        if let Some(secs) = self.stats.elapsed_on_finish {
            secs
        } else if let Some(started) = self.stats.typing_started_at {
            started.elapsed().as_secs_f64()
        } else {
            0.0
        }
    }
}
