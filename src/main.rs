mod app;
mod event;
mod layout;
mod ui;
mod words;

use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tachyonfx::{EffectManager, Interpolation, fx};

use app::{App, Screen};
use event::{AppEvent, is_quit, poll_event};
use layout::Layout;

fn main() -> Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    // Terminal restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new(Layout::discover_all());
    let mut effects: EffectManager<()> = EffectManager::default();
    let mut last_frame = Instant::now();

    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        terminal.draw(|frame| {
            ui::draw(frame, &app);
            let area = frame.area();
            effects.process_effects(elapsed.into(), frame.buffer_mut(), area);
        })?;

        if let Some(event) = poll_event(Duration::from_millis(16))? {
            match event {
                AppEvent::Key(key) => {
                    if is_quit(&key) {
                        break;
                    }

                    // Snapshot state before handling key
                    let screen_before = matches!(app.screen, Screen::Typing);

                    app.handle_key(key);

                    // Detect state changes and trigger effects
                    let screen_after = matches!(app.screen, Screen::Typing);

                    if !screen_before && screen_after {
                        // Entered typing screen
                        effects.add_effect(fx::coalesce((400, Interpolation::CubicOut)));
                    } else if screen_before && !screen_after {
                        // Returned to level select (Esc)
                        effects.add_effect(fx::dissolve((200, Interpolation::Linear)));
                    }
                }
                AppEvent::Resize => {} // ratatui handles redraw
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
