/// ファイルのフォーマット情報
pub struct FileInfo {
    pub format_name: String,
    pub has_tiff_header: bool,
    pub has_jpeg_marker: bool,
    pub byte_order: Option<String>,
}

/// バイトデータを解析してファイルフォーマット情報を返す
pub fn analyze(data: &[u8]) -> FileInfo {
    let has_tiff_header = detect_tiff(data);
    let has_jpeg_marker = detect_jpeg_soi(data);
    let byte_order = detect_byte_order(data);

    let format_name = if has_tiff_header && has_jpeg_marker {
        "RAW (TIFF + JPEG)".to_string()
    } else if has_tiff_header {
        "TIFF".to_string()
    } else if has_jpeg_marker {
        "JPEG".to_string()
    } else {
        "Unknown".to_string()
    };

    FileInfo {
        format_name,
        has_tiff_header,
        has_jpeg_marker,
        byte_order,
    }
}

/// TIFF ヘッダを検出する
/// Little-Endian: "II" + 0x002A
/// Big-Endian: "MM" + 0x002A
fn detect_tiff(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }

    let is_le = data[0] == b'I' && data[1] == b'I' && data[2] == 0x2A && data[3] == 0x00;
    let is_be = data[0] == b'M' && data[1] == b'M' && data[2] == 0x00 && data[3] == 0x2A;

    is_le || is_be
}

/// JPEG SOI マーカー (0xFF 0xD8) をデータ先頭から探す
fn detect_jpeg_soi(data: &[u8]) -> bool {
    if data.len() < 2 {
        return false;
    }

    // 先頭をチェック
    if data[0] == 0xFF && data[1] == 0xD8 {
        return true;
    }

    // TIFF構造内の埋め込みJPEGも検出するため、先頭64KBを走査
    let search_len = data.len().min(65536);
    for i in 0..search_len - 1 {
        if data[i] == 0xFF && data[i + 1] == 0xD8 {
            return true;
        }
    }

    false
}

/// バイトオーダーを検出する
fn detect_byte_order(data: &[u8]) -> Option<String> {
    if data.len() < 2 {
        return None;
    }

    if data[0] == b'I' && data[1] == b'I' {
        Some("Little-Endian".to_string())
    } else if data[0] == b'M' && data[1] == b'M' {
        Some("Big-Endian".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiff_little_endian() {
        let data = vec![b'I', b'I', 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
        let info = analyze(&data);

        assert!(info.has_tiff_header);
        assert_eq!(info.byte_order.as_deref(), Some("Little-Endian"));
        assert_eq!(info.format_name, "TIFF");
    }

    #[test]
    fn test_tiff_big_endian() {
        let data = vec![b'M', b'M', 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08];
        let info = analyze(&data);

        assert!(info.has_tiff_header);
        assert_eq!(info.byte_order.as_deref(), Some("Big-Endian"));
        assert_eq!(info.format_name, "TIFF");
    }

    #[test]
    fn test_jpeg_detection() {
        let data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        let info = analyze(&data);

        assert!(info.has_jpeg_marker);
        assert!(!info.has_tiff_header);
        assert_eq!(info.format_name, "JPEG");
    }

    #[test]
    fn test_tiff_with_embedded_jpeg() {
        let mut data = vec![b'I', b'I', 0x2A, 0x00];
        // TIFF ヘッダ後にJPEG SOIを埋め込む
        data.resize(100, 0x00);
        data[50] = 0xFF;
        data[51] = 0xD8;
        let info = analyze(&data);

        assert!(info.has_tiff_header);
        assert!(info.has_jpeg_marker);
        assert_eq!(info.format_name, "RAW (TIFF + JPEG)");
    }

    #[test]
    fn test_unknown_format() {
        let data = vec![0x00, 0x01, 0x02, 0x03];
        let info = analyze(&data);

        assert!(!info.has_tiff_header);
        assert!(!info.has_jpeg_marker);
        assert_eq!(info.format_name, "Unknown");
        assert!(info.byte_order.is_none());
    }
}
