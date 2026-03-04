use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub enum AppEvent {
    Key(KeyEvent),
    Resize,
}

pub fn poll_event(timeout: Duration) -> anyhow::Result<Option<AppEvent>> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key) => Ok(Some(AppEvent::Key(key))),
            Event::Resize(_, _) => Ok(Some(AppEvent::Resize)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn is_quit(key: &KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEventKind;

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new_with_kind(code, modifiers, KeyEventKind::Press)
    }

    #[test]
    fn ctrl_c_is_quit() {
        assert!(is_quit(&key(KeyCode::Char('c'), KeyModifiers::CONTROL)));
    }

    #[test]
    fn regular_keys_are_not_quit() {
        assert!(!is_quit(&key(KeyCode::Char('c'), KeyModifiers::NONE)));
        assert!(!is_quit(&key(KeyCode::Char('q'), KeyModifiers::NONE)));
        assert!(!is_quit(&key(KeyCode::Esc, KeyModifiers::NONE)));
    }
}
