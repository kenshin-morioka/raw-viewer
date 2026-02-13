mod app;
mod domain;
mod event;
mod infra;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::{bail, Result};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::state::AppState;
use crate::app::update;
use crate::event::{map_key, AppEvent};
use crate::infra::file_loader;
use crate::ui::layout;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        bail!("Usage: raw-viewer <file-path>");
    }
    let file_path = &args[1];

    let data = file_loader::load_file(file_path)?;
    let mut state = AppState::new(data, file_path.clone());

    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal, &mut state);
    restore_terminal(&mut terminal)?;

    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            layout::render(frame, state);
        })?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if let Some(app_event) = map_key(key_event.code) {
                    if matches!(app_event, AppEvent::Quit) {
                        return Ok(());
                    }
                    update::update(state, app_event);
                }
            }
        }
    }
}
