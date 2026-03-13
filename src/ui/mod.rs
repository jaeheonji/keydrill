pub mod color_cycle;
pub mod keyboard;
pub mod level_select;
pub mod results;
pub mod typing;

use std::time::Duration;

use ratatui::Frame;

use crate::app::{App, Screen};
use crate::config::theme::Theme;

pub fn draw(frame: &mut Frame, app: &App, theme: &Theme, elapsed: Duration, effect_enabled: bool, palette: &[(u8, u8, u8)]) {
    match app.screen {
        Screen::LevelSelect => level_select::draw(frame, app, theme),
        Screen::Typing => typing::draw(frame, app, theme, elapsed, effect_enabled, palette),
        Screen::Results => results::draw(frame, app, theme),
    }
}
