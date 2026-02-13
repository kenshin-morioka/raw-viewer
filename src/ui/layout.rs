use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::state::AppState;
use crate::ui::widgets::{hex_view_widget, info_panel_widget};

/// メインレイアウトを描画する（水平2分割: hex view 75% + info panel 25%）
pub fn render(frame: &mut Frame, state: &AppState) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(75),
        Constraint::Percentage(25),
    ])
    .split(frame.size());

    let visible_lines = chunks[0].height.saturating_sub(2) as usize; // ボーダー分を引く
    let hex_view = hex_view_widget(state, visible_lines);
    let info_panel = info_panel_widget(state);

    frame.render_widget(hex_view, chunks[0]);
    frame.render_widget(info_panel, chunks[1]);
}
