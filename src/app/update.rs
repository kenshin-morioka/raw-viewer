use crate::app::state::AppState;
use crate::event::AppEvent;

/// イベントに基づいてアプリケーション状態を更新する。
/// 状態変更のみ行い、IO・描画は行わない。
pub fn update(state: &mut AppState, event: AppEvent) {
    let total_lines = state.total_lines();
    if total_lines == 0 {
        return;
    }
    let max_cursor = total_lines - 1;

    match event {
        AppEvent::MoveUp => {
            state.cursor_offset = state.cursor_offset.saturating_sub(1);
        }
        AppEvent::MoveDown => {
            if state.cursor_offset < max_cursor {
                state.cursor_offset += 1;
            }
        }
        AppEvent::PageUp => {
            state.cursor_offset = state.cursor_offset.saturating_sub(20);
        }
        AppEvent::PageDown => {
            state.cursor_offset = (state.cursor_offset + 20).min(max_cursor);
        }
        AppEvent::Quit => {}
    }

    // スクロール追従: カーソルが表示範囲外に出たら追従させる
    if state.cursor_offset < state.scroll_offset {
        state.scroll_offset = state.cursor_offset;
    }
    // visible_lines は描画時に決まるため、ここでは仮に40行とする
    let visible_lines = 40;
    if state.cursor_offset >= state.scroll_offset + visible_lines {
        state.scroll_offset = state.cursor_offset - visible_lines + 1;
    }
}
