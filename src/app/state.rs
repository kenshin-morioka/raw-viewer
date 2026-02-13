use crate::domain::raw_parser::{self, FileInfo};

/// アプリケーションの状態
pub struct AppState {
    pub data: Vec<u8>,
    pub file_path: String,
    pub file_size: usize,
    pub cursor_offset: usize,
    pub scroll_offset: usize,
    pub file_info: FileInfo,
}

impl AppState {
    /// 新しい AppState を生成する。初期化時にファイル解析結果をキャッシュする。
    pub fn new(data: Vec<u8>, file_path: String) -> Self {
        let file_size = data.len();
        let file_info = raw_parser::analyze(&data);

        Self {
            data,
            file_path,
            file_size,
            cursor_offset: 0,
            scroll_offset: 0,
            file_info,
        }
    }

    /// データ全体の行数を返す（16バイト/行）
    pub fn total_lines(&self) -> usize {
        (self.file_size + 15) / 16
    }
}
