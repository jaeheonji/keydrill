mod app;
mod config;
mod event;
mod layout;
mod ui;
mod utils;
mod words;

use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tachyonfx::{EffectManager, Interpolation, fx};

use app::{App, Screen};
use config::Config;
use event::{AppEvent, is_quit, poll_event};
use layout::Layout;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    /// Path to config file
    #[arg(long)]
    config: Option<PathBuf>,
}

fn setup_debug_logging() -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    let uid = fs::metadata("/proc/self")?.uid();
    let dir = format!("/tmp/keydrill-{uid}");
    fs::create_dir_all(&dir)?;
    let log_path = format!("{dir}/keydrill.log");
    let log_file = fs::File::create(&log_path)?;

    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    eprintln!("Debug log: {log_path}");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.debug {
        setup_debug_logging()?;
    }

    let config = Config::load(cli.config)?;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &config);

    // Terminal restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, config: &Config) -> Result<()> {
    let mut app = App::new(Layout::discover_all(&config.general.layouts), config);
    let mut effects: EffectManager<()> = EffectManager::default();
    let mut last_frame = Instant::now();
    let app_start = Instant::now();
    let palette = config.effect.resolve_palette();

    loop {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();

        terminal.draw(|frame| {
            ui::draw(frame, &app, &config.theme, app_start.elapsed(), config.effect.enabled, &palette);
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
