use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::state::AppState;
use crate::domain::hex_view;

/// hex view ウィジェットを生成する
pub fn hex_view_widget(state: &AppState, visible_lines: usize) -> Paragraph<'static> {
    let start_byte_offset = state.scroll_offset * 16;
    let lines = hex_view::generate_lines(&state.data, start_byte_offset, visible_lines);

    let styled_lines: Vec<Line<'static>> = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let line_index = state.scroll_offset + i;
            if line_index == state.cursor_offset {
                Line::from(Span::styled(
                    line,
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(line)
            }
        })
        .collect();

    Paragraph::new(styled_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Hex View "),
    )
}

/// info panel ウィジェットを生成する
pub fn info_panel_widget(state: &AppState) -> Paragraph<'static> {
    let cursor_byte_offset = state.cursor_offset * 16;
    let byte_order_display = state
        .file_info
        .byte_order
        .as_deref()
        .unwrap_or("N/A");

    let info_text = vec![
        Line::from(format!("File: {}", state.file_path)),
        Line::from(format!("Size: {} bytes", state.file_size)),
        Line::from(format!(
            "Size: {:.2} KB",
            state.file_size as f64 / 1024.0
        )),
        Line::from(""),
        Line::from(format!("Format: {}", state.file_info.format_name)),
        Line::from(format!("TIFF: {}", if state.file_info.has_tiff_header { "Yes" } else { "No" })),
        Line::from(format!("JPEG: {}", if state.file_info.has_jpeg_marker { "Yes" } else { "No" })),
        Line::from(format!("Byte Order: {byte_order_display}")),
        Line::from(""),
        Line::from(format!("Cursor: line {}", state.cursor_offset)),
        Line::from(format!("Offset: 0x{cursor_byte_offset:08X}")),
        Line::from(format!(
            "Total: {} lines",
            state.total_lines()
        )),
        Line::from(""),
        Line::from("--- Controls ---"),
        Line::from("Up/Down: Scroll 1 line"),
        Line::from("PgUp/PgDn: Scroll 20 lines"),
        Line::from("q: Quit"),
    ];

    Paragraph::new(info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Info "),
    )
}
