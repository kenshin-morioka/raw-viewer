use anyhow::{Context, Result};

/// ファイルを一括読み込みしてバイト列を返す
pub fn load_file(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path).with_context(|| format!("Failed to read file: {path}"))
}
