/// 16バイト/行の hex dump 行を生成する。
/// 8バイトでグループ区切り、ASCII表示付き。
pub fn generate_lines(data: &[u8], start_offset: usize, max_lines: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for i in 0..max_lines {
        let offset = start_offset + i * 16;
        if offset >= data.len() {
            break;
        }

        let end = (offset + 16).min(data.len());
        let chunk = &data[offset..end];

        let mut hex_part = String::new();
        let mut ascii_part = String::new();

        for (j, &byte) in chunk.iter().enumerate() {
            if j == 8 {
                hex_part.push(' ');
            }
            if j > 0 {
                hex_part.push(' ');
            }
            hex_part.push_str(&format!("{byte:02X}"));

            if byte.is_ascii_graphic() || byte == b' ' {
                ascii_part.push(byte as char);
            } else {
                ascii_part.push('.');
            }
        }

        // 未満行のパディング
        let remaining = 16 - chunk.len();
        for j in 0..remaining {
            let pos = chunk.len() + j;
            if pos == 8 {
                hex_part.push(' ');
            }
            hex_part.push_str("   ");
            ascii_part.push(' ');
        }

        lines.push(format!("{offset:08X}  {hex_part}  |{ascii_part}|"));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_16_byte_line() {
        let data: Vec<u8> = (0x00..0x10).collect();
        let lines = generate_lines(&data, 0, 1);

        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "00000000  00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F  |................|"
        );
    }

    #[test]
    fn test_partial_line_padding() {
        let data = vec![0x41, 0x42, 0x43]; // "ABC"
        let lines = generate_lines(&data, 0, 1);

        assert_eq!(lines.len(), 1);
        // hex_part は常に48文字にパディングされる
        assert!(lines[0].starts_with("00000000  41 42 43"));
        assert!(lines[0].ends_with("|ABC             |"));
        // 全体長は完全な16バイト行と同じ78文字
        assert_eq!(lines[0].len(), 78);
    }

    #[test]
    fn test_multiple_lines() {
        let data: Vec<u8> = (0..32).collect();
        let lines = generate_lines(&data, 0, 10);

        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("00000000"));
        assert!(lines[1].starts_with("00000010"));
    }

    #[test]
    fn test_start_offset() {
        let data: Vec<u8> = (0..48).collect();
        let lines = generate_lines(&data, 16, 2);

        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("00000010"));
        assert!(lines[1].starts_with("00000020"));
    }

    #[test]
    fn test_ascii_display() {
        let data = b"Hello, World!!\x00\x01";
        let lines = generate_lines(data, 0, 1);

        assert_eq!(lines.len(), 1);
        assert!(lines[0].ends_with("|Hello, World!!..|"));
    }
}
