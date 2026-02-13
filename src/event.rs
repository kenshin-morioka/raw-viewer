use crossterm::event::KeyCode;

/// アプリケーションイベント
pub enum AppEvent {
    Quit,
    MoveUp,
    MoveDown,
    PageUp,
    PageDown,
}

/// キー入力をアプリケーションイベントに変換する
pub fn map_key(key: KeyCode) -> Option<AppEvent> {
    match key {
        KeyCode::Char('q') => Some(AppEvent::Quit),
        KeyCode::Up => Some(AppEvent::MoveUp),
        KeyCode::Down => Some(AppEvent::MoveDown),
        KeyCode::PageUp => Some(AppEvent::PageUp),
        KeyCode::PageDown => Some(AppEvent::PageDown),
        _ => None,
    }
}
