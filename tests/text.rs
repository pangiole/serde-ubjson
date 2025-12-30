
/// Generates a multilingual text string that, when UTF-8 encoded, has exactly `byte_count` bytes.
/// Uses a mix of ASCII (1 byte), Latin Extended (2 bytes), CJK (3 bytes), and Emoji (4 bytes).
pub fn generate(byte_count: usize) -> (String, Vec<u8>) {
    let mut string = String::new();
    let mut bytes_added = 0;

    // Character sets with their UTF-8 byte sizes
    let chars_1byte = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']; // ASCII
    let chars_2byte = ['á', 'é', 'ñ', 'ü', 'ø', 'ć', 'ž', 'ł', 'ş', 'ğ']; // Latin Extended
    let chars_3byte = ['中', '日', '한', '語', '文', '字', '本', '国', '語', '言']; // CJK
    let chars_4byte = ['😀', '🌍', '🚀', '🎨', '🔥', '💻', '🎵', '🌟', '⚡', '🎯']; // Emoji

    let mut idx = 0;
    while bytes_added < byte_count {
        let remaining = byte_count - bytes_added;

        // Choose character based on remaining bytes and cycling pattern
        let ch = match (remaining, idx % 4) {
            (1, _) => chars_1byte[idx % chars_1byte.len()],
            (2, _) => chars_2byte[idx % chars_2byte.len()],
            (3, _) => chars_3byte[idx % chars_3byte.len()],
            (_, 0) if remaining >= 4 => chars_4byte[idx % chars_4byte.len()],
            (_, 1) if remaining >= 3 => chars_3byte[idx % chars_3byte.len()],
            (_, 2) if remaining >= 2 => chars_2byte[idx % chars_2byte.len()],
            _ => chars_1byte[idx % chars_1byte.len()],
        };

        string.push(ch);
        bytes_added += ch.len_utf8();
        idx += 1;
    }

    (string.clone(), expected_ubj(string.as_bytes()))
}

fn expected_ubj(bytes: &[u8]) -> Vec<u8> {
    let len = bytes.len();
    let mut expected = Vec::with_capacity(1 + 5 + len); // worst-case for i32 length
    expected.push(0x53); // 'S'
    if len <= u8::MAX as usize {
        expected.push(0x55); // 'U'
        expected.push(len as u8);
    } else if len <= i16::MAX as usize {
        expected.push(0x49); // 'I'
        expected.push(((len >> 8) & 0xFF) as u8);
        expected.push((len & 0xFF) as u8);
    } else if len <= i32::MAX as usize {
        expected.push(0x6C); // 'l'
        expected.push(((len >> 24) & 0xFF) as u8);
        expected.push(((len >> 16) & 0xFF) as u8);
        expected.push(((len >> 8) & 0xFF) as u8);
        expected.push((len & 0xFF) as u8);
    } else {
        assert!(
            len <= i64::MAX as usize,
            "test string too long for i64 length"
        );
        expected.push(0x4C); // 'L'
        expected.push(((len >> 56) & 0xFF) as u8);
        expected.push(((len >> 48) & 0xFF) as u8);
        expected.push(((len >> 40) & 0xFF) as u8);
        expected.push(((len >> 32) & 0xFF) as u8);
        expected.push(((len >> 24) & 0xFF) as u8);
        expected.push(((len >> 16) & 0xFF) as u8);
        expected.push(((len >> 8) & 0xFF) as u8);
        expected.push((len & 0xFF) as u8);
    }
    expected.extend_from_slice(bytes);
    expected
}