pub mod keyboard;
pub mod level_select;
pub mod results;
pub mod typing;

use ratatui::Frame;

use crate::app::{App, Screen};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::LevelSelect => level_select::draw(frame, app),
        Screen::Typing => typing::draw(frame, app),
        Screen::Results => results::draw(frame, app),
    }
}
