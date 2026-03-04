pub mod keyboard;
pub mod level_select;
pub mod results;
pub mod typing;

use ratatui::Frame;

use crate::app::{App, Screen};
use crate::config::theme::Theme;

pub fn draw(frame: &mut Frame, app: &App, theme: &Theme) {
    match app.screen {
        Screen::LevelSelect => level_select::draw(frame, app, theme),
        Screen::Typing => typing::draw(frame, app, theme),
        Screen::Results => results::draw(frame, app, theme),
    }
}
