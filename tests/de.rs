// See http://dmitry-ra.github.io/ubjson-test-suite/json-converter.html

use serde_ubj::*;

#[path = "model.rs"]
mod model;

#[path = "text.rs"]
mod text;


#[cfg(feature = "std")]
fn buf_reader_of(bytes: &[u8], buffer_capacity: usize) -> std::io::BufReader<std::io::Cursor<&[u8]>> {
    std::io::BufReader::with_capacity(buffer_capacity, std::io::Cursor::new(bytes))
}

#[cfg(all(not(feature = "std"), feature = "embedded-io"))]
fn buf_reader_of(bytes: &[u8], _capacity: usize) -> &[u8] {
    bytes
}


macro_rules! assert_deserialize_value_ok {
    ($bytes:expr, $t:ty, $expected:expr) => {
        assert_deserialize_value_ok!($bytes, $t, $expected, 8192)
    };
    ($bytes:expr, $t:ty, $expected:expr, $capacity:expr) => {
        let mut reader = buf_reader_of($bytes, $capacity);
        let result: UbjResult<$t> = from_buf_reader(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), $expected);
    };
}

macro_rules! assert_deserialize_split_utf8_ok {
    ($bytes:expr, $capacity:expr) => {
        let expected_text = String::from_utf8((&$bytes[3..]).to_vec()).unwrap();
        let mut buf_reader = buf_reader_of($bytes, $capacity);
        let result: UbjResult<String> = from_buf_reader(&mut buf_reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_text);
    };
}


macro_rules! assert_deserialize_value_err {
    ($bytes:expr, $t:ty, $expected:pat) => {
        let mut buf_reader = buf_reader_of($bytes, 8192);
        let result: UbjResult<$t> = from_buf_reader(&mut buf_reader);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), $expected));
    };
}


// ---------------------------------------------------------------------------------
// S C A L A R    values
// ---------------------------------------------------------------------------------

#[test]
fn deserialize_to_unit() {
    assert_deserialize_value_ok! (&[0x5A], (), ());
    assert_deserialize_value_err!(&[    ], (), UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF], (), UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_to_bool() {
    assert_deserialize_value_ok! (&[0x54], bool, true);
    assert_deserialize_value_ok! (&[0x46], bool, false);
    assert_deserialize_value_err!(&[    ], bool, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF], bool, UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_to_i8() {
    assert_deserialize_value_ok! (&[0x69, 0x85], i8, -123_i8);
    assert_deserialize_value_ok! (&[0x69, 0x7B], i8, 123_i8);
    assert_deserialize_value_err!(&[0x69,     ], i8, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF, 0x85], i8, UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_to_u8() {
    assert_deserialize_value_ok! (&[0x55, 0x7B], u8, 123_u8);
    assert_deserialize_value_ok! (&[0x55, 0xFE], u8, 254_u8);
    assert_deserialize_value_err!(&[0x55,     ], u8, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF, 0x7B], u8, UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_to_i16() {
    assert_deserialize_value_ok! (&[0x49, 0x80, 0x44], i16, -32700_i16);
    assert_deserialize_value_ok! (&[0x49, 0x7F, 0xBC], i16, 32700_i16);
    assert_deserialize_value_err!(&[0x49, 0x7F,     ], i16, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF, 0x80, 0x44], i16, UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_to_i32() {
    assert_deserialize_value_ok! (&[0x6C, 0xB5, 0xA4, 0xE9, 0x00], i32, -1247483648_i32);
    assert_deserialize_value_ok! (&[0x6C, 0xFF, 0xFF, 0x02, 0x18], i32, -65000_i32);
    assert_deserialize_value_ok! (&[0x6C, 0x00, 0x00, 0xFD, 0xE8], i32, 65000_i32);
    assert_deserialize_value_ok! (&[0x6C, 0x4A, 0x5B, 0x17, 0x00], i32, 1247483648_i32);
    assert_deserialize_value_err!(&[0x6C, 0x4A, 0x5B,           ], i32, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF, 0x4A, 0x5B, 0x17, 0x00], i32, UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_1_2_1_split_bytes_to_i32() {
    let ubj_bytes: &[u8] = &[
        // chunk #1 of 7 bytes
        0x6C, 0x4A,
        // chunk #2 of 2 bytes
        0x5B, 0x17,
        // chunk #3 of 1 byte
        0x00,
    ];
    assert_deserialize_value_ok!(ubj_bytes, i32, 1247483648_i32, 2);
}


#[test]
fn deserialize_to_i64() {
    assert_deserialize_value_ok! (&[0x4C, 0xFF, 0xFC, 0xB9, 0x23, 0xA2, 0x9C, 0x77, 0x9B], i64, -922337203685477_i64);
    assert_deserialize_value_ok! (&[0x4C, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x06], i64, -4294967290_i64);
    assert_deserialize_value_ok! (&[0x4C, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFA], i64, 4294967290_i64);
    assert_deserialize_value_ok! (&[0x4C, 0x00, 0x03, 0x46, 0xDC, 0x5D, 0x63, 0x88, 0x65], i64, 922337203685477_i64);
    assert_deserialize_value_err!(&[0x4C, 0x00, 0x03, 0x46, 0xDC,                       ], i64, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0xFF, 0x00, 0x03, 0x46, 0xDC, 0x5D, 0x63, 0x88, 0x65], i64, UbjError::UnexpectedMarker(0xFF));
}

#[test]
fn deserialize_6_2_split_bytes_to_i64() {
    let ubj_bytes: &[u8] = &[
        // chunk #1 of 7 bytes
        0x4C, 0xFF, 0xFC, 0xB9, 0x23, 0xA2, 0x9C,
        // chunk #2 of 2 bytes
        0x77, 0x9B,
    ];
    assert_deserialize_value_ok!(ubj_bytes, i64, -922337203685477_i64, 7);
}
#[test]
fn deserialize_to_i128() {
    assert_deserialize_value_err! (&[], i128, UbjError::Unsupported("i128"));
}
#[test]
fn deserialize_to_u16() {
    assert_deserialize_value_err! (&[], u16, UbjError::Unsupported("u16"));
}
#[test]
fn deserialize_to_u32() {
    assert_deserialize_value_err! (&[], u32, UbjError::Unsupported("u32"));
}
#[test]
fn deserialize_to_u64() {
    assert_deserialize_value_err! (&[], u64, UbjError::Unsupported("u64"));
}
#[test]
fn deserialize_to_u128() {
    assert_deserialize_value_err! (&[], u128, UbjError::Unsupported("u128"));
}

#[test]
fn deserialize_to_f32() {
    assert_deserialize_value_ok! (&[0x64, 0x3E, 0x20, 0x00, 0x00], f32, 0.15625_f32);
    assert_deserialize_value_err!(&[0x64, 0x3E, 0x20,           ], f32, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0x53, 0x3E, 0x20, 0x00, 0x00], f32, UbjError::UnexpectedMarker(0x53));
}

#[test]
fn deserialize_3_1_split_bytes_to_f32() {
    let ubj_bytes: &[u8] = &[
        // chunk #1 of 4 bytes
        0x64, 0xC2, 0xED, 0x40,
        // chunk #2 of 1 byte
        0x01
    ];
    assert_deserialize_value_ok!(ubj_bytes, f32, -118.625008_f32, 4);
}

#[test]
fn deserialize_to_f64() {
    assert_deserialize_value_ok! (&[0x44, 0x41, 0x70, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00], f64, 16777216.125_f64);
    assert_deserialize_value_err!(&[0x44, 0x41, 0x70, 0x00, 0x00,                       ], f64, UbjError::UnexpectedEof);
    assert_deserialize_value_err!(&[0x53, 0x41, 0x70, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00], f64, UbjError::UnexpectedMarker(0x53));
}

#[test]
fn deserialize_5_3_split_bytes_to_f64() {
    let ubj_bytes: &[u8] = &[
        // chunk #1 of 6 bytes
        0x44, 0x3F, 0xF3, 0xBE, 0x76, 0xC8,
        // chunk #2 of 3 bytes
        0xB4, 0x39, 0x58
    ];
    assert_deserialize_value_ok!(ubj_bytes, f64, 1.234_f64, 6);
}

#[test]
fn deserialize_to_char() {
    assert_deserialize_value_ok! (&[0x43, 0x48], char, 'H');
    assert_deserialize_value_err!(&[0x43, 0xFF], char, UbjError::CharNotAscii(0xFF));
    assert_deserialize_value_err!(&[0xFF, 0x48], char, UbjError::UnexpectedMarker(0xFF));
    assert_deserialize_value_err!(&[          ], char, UbjError::UnexpectedEof);
}


// This scenario covers UTF-8 characters that are valid and can be easily deserialized
// as the buffer capacity is such that the "split data" scenario never occurs.
#[test]
fn deserialize_evenly_split_bytes_to_utf8_string() {
    let ubj_bytes = &[
        //  chunk #1 of 15 bytes
        //  [S]   [i]   [29]  [¡.......]  [R]   [u]   [s]   [t]   [ ]   [i]   [s]   [ ]   [s]   [a]
            0x53, 0x69, 0x1D, 0xC2, 0xA1, 0x52, 0x75, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x61,

        //  chunk #2 of 15 bytes
        //  [f]   [e]   [ ]   [🦀..................]  [ ]   [a]   [n]   [d]   [ ]   [f]   [a]   [s]
            0x66, 0x65, 0x20, 0xF0, 0x9F, 0xA6, 0x80, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x66, 0x61, 0x73,

        //  chunk 3 of 2 bytes
        //  [t]   [!]
            0x74, 0x21
    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 15);
}


// This scenario covers UTF-8 characters requiring 2 bytes (such as 'é' and '£') that got split
// across two chunks: 1 byte in the current chunk and 1 byte in the next chunk.
#[test]
fn deserialize_1_1_split_bytes_to_utf8_string() {
    // "The café costs £15 to enter";
    let ubj_bytes = &[
    //  chunk #1 of 20 bytes
    //  [S]   [i]   [29]  [T]   [h]   [e]   [ ]   [c]   [a]   [f]   [é       ]  [ ]   [c]   [o]   [s]   [t]   [s]   [ ]   [£...
        0x53, 0x69, 0x1D, 0x54, 0x68, 0x65, 0x20, 0x63, 0x61, 0x66, 0xC3, 0xA9, 0x20, 0x63, 0x6F, 0x73, 0x74, 0x73, 0x20, 0xC2,

    //  chunk #2 of 12 bytes
    //  ...]  [1]   [5]   [ ]   [t]   [o]   [ ]   [e]   [n]   [t]   [e]   [r]
        0xA3, 0x31, 0x35, 0x20, 0x74, 0x6F, 0x20, 0x65, 0x6E, 0x74, 0x65, 0x72,
    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 20);
}


// This scenario covers UTF-8 characters requiring 3 bytes (such as the '⚡' High Voltage) that got split
// across two chunks: 1 byte in the current chunk and 2 bytes in the next chunk.
#[test]
fn deserialize_1_2_split_bytes_to_utf8_string() {
    // "Our team is ⚡ fast and slim"
    let ubj_bytes = &[
        //  chunk #1 of 16 bytes
        //  [S]   [i]   [29]  [O]   [u]   [r]   [ ]   [t]   [e]   [a]   [m]   [ ]   [i]   [s]   [ ]   [⚡...
            0x53, 0x69, 0x1D, 0x4F, 0x75, 0x72, 0x20, 0x74, 0x65, 0x61, 0x6D, 0x20, 0x69, 0x73, 0x20, 0xE2,


        //  chunk #2 of 16 bytes
        //  .........]  [ ]   [f]   [a]   [s]   [t]   [ ]   [a]   [n]   [d]   [ ]   [s]   [l]   [i]   [m]
            0x9A, 0xA1, 0x20, 0x66, 0x61, 0x73, 0x74, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x73, 0x6C, 0x69, 0x6D

    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 16);
}


// This scenario covers UTF-8 characters requiring 3 bytes (such as the '⚡' High Voltage) that got split
// across two chunks: 2 bytes in the current chunk and 1 byte in the next chunk.
#[test]
fn deserialize_2_1_split_bytes_to_utf8_string() {
    // "Our team is ⚡ fast and slim"
    let ubj_bytes = &[
        //  chunk #1 of 17 bytes
        //  [S]   [i]   [29]  [O]   [u]   [r]   [ ]   [t]   [e]   [a]   [m]   [ ]   [i]   [s]   [ ]   [⚡.......
            0x53, 0x69, 0x1D, 0x4F, 0x75, 0x72, 0x20, 0x74, 0x65, 0x61, 0x6D, 0x20, 0x69, 0x73, 0x20, 0xE2, 0x9A,


        //  chunk #2 of 15 bytes
        //  ...]  [ ]   [f]   [a]   [s]   [t]   [ ]   [a]   [n]   [d]   [ ]   [s]   [l]   [i]   [m]
            0xA1, 0x20, 0x66, 0x61, 0x73, 0x74, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x73, 0x6C, 0x69, 0x6D

    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 17);
}


// This scenario covers UTF-8 characters requiring 4 bytes (such as the '🦀' Crab) that got split
// across two chunks: 3 bytes in the current chunk and 1 byte in the next chunk.
#[test]
fn deserialize_3_1_split_bytes_to_utf8_string() {
    // "¡Rust is safe 🦀and fast!"
    let ubj_bytes = &[
        //  chunk #1 of 21 bytes
        //  [S]   [i]   [29]  [¡       ]  [R]   [u]   [s]   [t]   [ ]   [i]   [s]   [ ]   [s]   [a]   [f]   [e]   [ ]   [🦀.............
            0x53, 0x69, 0x1D, 0xC2, 0xA1, 0x52, 0x75, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x61, 0x66, 0x65, 0x20, 0xF0, 0x9F, 0xA6,

        //  chunk #2 of 11 bytes
        //  ...]  [ ]   [a]   [n]   [d]   [ ]   [f]   [a]   [s]   [t]   [!]
            0x80, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x66, 0x61, 0x73, 0x74, 0x21
    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 21);
}


// This scenario covers UTF-8 characters requiring 4 bytes (such as the '🦀' Crab) that got split
// across two chunks: 2 bytes in the current chunk and 2 bytes in the next chunk.
#[test]
fn deserialize_2_2_split_bytes_to_utf8_string() {
    // "¡Rust is safe 🦀and fast!"
    let ubj_bytes = &[
        //  chunk #1 of 20 bytes
        //  [S]   [i]   [29]  [¡       ]  [R]   [u]   [s]   [t]   [ ]   [i]   [s]   [ ]   [s]   [a]   [f]   [e]   [ ]   [🦀.......
            0x53, 0x69, 0x1D, 0xC2, 0xA1, 0x52, 0x75, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x61, 0x66, 0x65, 0x20, 0xF0, 0x9F,

        //  chunk #2 of 12 bytes
        //  ..........]  [ ]   [a]   [n]   [d]   [ ]   [f]   [a]   [s]   [t]   [!]
            0xA6, 0x80, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x66, 0x61, 0x73, 0x74, 0x21
    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 20);
}


// This scenario covers UTF-8 characters requiring 4 bytes (such as the '🦀' Crab) that got split
// across two chunks: 1 byte in the current chunk and 3 bytes in the next chunk.
#[test]
fn deserialize_1_3_split_bytes_to_utf8_string() {
    // "¡Rust is safe 🦀and fast!"
    let ubj_bytes = &[
        //  chunk #1 of 19 bytes
        //  [S]   [i]   [29]  [¡       ]  [R]   [u]   [s]   [t]   [ ]   [i]   [s]   [ ]   [s]   [a]   [f]   [e]   [ ]   [🦀...
            0x53, 0x69, 0x1D, 0xC2, 0xA1, 0x52, 0x75, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x61, 0x66, 0x65, 0x20, 0xF0,

        //  chunk #2 of 13 bytes
        //  ............... ]  [ ]   [a]   [n]   [d]   [ ]   [f]   [a]   [s]   [t]   [!]
            0x9F, 0xA6, 0x80, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x66, 0x61, 0x73, 0x74, 0x21
    ];
    assert_deserialize_split_utf8_ok!(ubj_bytes, 19);
}


#[test]
fn deserialize_45_contiguous_bytes_to_utf8_string() {
    let (text, ubj) = text::generate(45);
    assert_deserialize_value_ok!(ubj.as_slice(), String, text);
}

#[test]
fn deserialize_230_contiguous_bytes_to_utf8_string() {
    let (text, ubj) = text::generate(230);
    assert_deserialize_value_ok!(ubj.as_slice(), String, text);
}

#[test]
fn deserialize_15300_contiguous_bytes_to_utf8_string() {
    let (text, ubj) = text::generate(15300);
    assert_deserialize_value_ok!(ubj.as_slice(), String, text);
}

#[test]
fn deserialize_7483648_contiguous_bytes_to_utf8_string() {
    let (text, ubj) = text::generate(7483648);
    assert_deserialize_value_ok!(ubj.as_slice(), String, text);
}

#[test]
#[ignore]
fn deserialize_2147483648_contiguous_bytes_to_utf8_string() {
    let (text, ubj) = text::generate(2147483648);
    assert_deserialize_value_ok!(ubj.as_slice(), String, text);
}


// ---------------------------------------------------------------------------------
// C O M P O U N D   values
// ---------------------------------------------------------------------------------

#[test]
fn deserialize_to_none() {
    assert_deserialize_value_ok!(&[0x5A], Option<()>, None);
}

#[test]
fn deserialize_to_some_bool() {
    assert_deserialize_value_ok!(&[0x54], Option<bool>, Some(true));
    assert_deserialize_value_ok!(&[0x46], Option<bool>, Some(false));
}


// SEQUENCE-LIKE
// ---------

#[test]
fn deserialize_to_vector() {
    let ubj_bytes = [
    //   [[]
        0x5B,
            0x69, 0x0C,
            0x69, 0x40,
            0x69, 0x7B,
    //   []]
        0x5D
    ];
    assert_deserialize_value_ok!(&ubj_bytes, Vec<i8>, vec![12_i8, 64_i8, 123_i8]);
}

#[test]
fn deserialize_to_array() {
    let ubj_bytes = [
        //   [[]
        0x5B,
            0x69, 0x0C,
            0x69, 0x40,
            0x69, 0x7B,
        //   []]
        0x5D
    ];
    assert_deserialize_value_ok!(&ubj_bytes, [i8; 3], [12_i8, 64_i8, 123_i8]);
}

#[test]
fn deserialize_to_tuple() {
    let ubj_bytes = [
        //   [[]
        0x5B,
        0x69, 0x0C,
        0x69, 0x40,
        0x69, 0x7B,
        //   []]
        0x5D
    ];
    assert_deserialize_value_ok!(&ubj_bytes, (i8, i8, i8), (12_i8, 64_i8, 123_i8));
}


// STRUCTS
// -------

#[test]
fn deserialize_to_unit_struct() {
    use model::MyUnitStruct;
    assert_deserialize_value_ok!(&[0x5A], MyUnitStruct, MyUnitStruct{});
}

#[test]
fn deserialize_to_newtype_struct() {
    use model::MyNewtypeStruct;
    assert_deserialize_value_ok!(&[0x69, 0x7B], MyNewtypeStruct, MyNewtypeStruct(123_i8));
}


#[test]
fn deserialize_to_tuple_struct() {
    use model::MyTupleStruct;
    assert_deserialize_value_ok!(&[
        0x5B,
            0x69, 0x7B,
            0x49, 0x7F, 0xBC,
            0x6C, 0x4A, 0x5B, 0x17, 0x00,
        0x5D,
    ], MyTupleStruct, MyTupleStruct(123_i8, 32700_i16, 1247483648_i32));
}

#[test]
fn deserialize_to_struct() {
    use model::MyFieldsStruct;
    assert_deserialize_value_ok!(&[
        0x7B,
            0x55, 0x01, 0x78,    0x69, 0x7B,
            0x55, 0x01, 0x79,    0x46,
            0x55, 0x01, 0x7A,    0x53, 0x55, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65,
        0x7D,
    ], MyFieldsStruct, MyFieldsStruct { x: 123, y: false, z: String::from("value") });
}



// ENUMS
// ----------

#[test]
fn deserialize_to_unit_variant() {
    use model::MyEnum;
    assert_deserialize_value_ok!(&[
        0x7B,
    //       [u]  [13]   [M]   [y]   [U]   [n]   [i]   [t]   [V]  [a]   [r]   [i]    [a]   [n]  [t]
            0x55, 0x0D, 0x4D, 0x79, 0x55, 0x6E, 0x69, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
    //       [Z]
            0x5A,
        0x7D
    ], MyEnum, MyEnum::MyUnitVariant);
}

#[test]
fn deserialize_to_newtype_variant() {
    use model::MyEnum;
    assert_deserialize_value_ok!(&[
        0x7B,
        //   variant identifier
        //   [u]  [16]   [M]   [y]   [N]   [e]   [w]   [t]   [y]   [p]   [e]   [V]   [a]   [r]   [i]   [a]   [n]   [t]
            0x55, 0x10, 0x4D, 0x79, 0x4E, 0x65, 0x77, 0x74, 0x79, 0x70, 0x65, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
        //   variant data
        //   [S]   [u]   [5]   [v]   [a]   [l]   [u]   [e]
            0x53, 0x55, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65,
        0x7D
    ], MyEnum, MyEnum::MyNewtypeVariant(String::from("value")));
}

#[test]
fn deserialize_to_tuple_variant() {
    use model::MyEnum;
    assert_deserialize_value_ok!(&[
        0x7B,
        //   variant identifier
        //   [u]  [14]   [M]   [y]   [T]   [u]   [p]   [l]   [e]   [V]   [a]   [r]   [i]   [a]   [n]  [t]
            0x55, 0x0E, 0x4D, 0x79, 0x54, 0x75, 0x70, 0x6C, 0x65, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
        //   variant data (sequence)
            0x5B,
                0x69, 0x7B,
                0x49, 0x7F, 0xBC,
                0x6C, 0x4A, 0x5B, 0x17, 0x00,
            0x5D,
        0x7D,
    ], MyEnum, MyEnum::MyTupleVariant(123_i8, 32700_i16, 1247483648_i32));
}


#[test]
fn deserialize_to_struct_variant() {
    use model::MyEnum;
    assert_deserialize_value_ok!(&[
        0x7B,
        //   variant identifier
        //   [u]  [15]
            0x55, 0x0F, 0x4D, 0x79, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
        //   variant data (struct)
            0x7B,
                0x55, 0x01, 0x78,    0x69, 0x7B,
                0x55, 0x01, 0x79,    0x46,
                0x55, 0x01, 0x7A,    0x53, 0x55, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65,
            0x7D,
        0x7D
    ], MyEnum, MyEnum::MyStructVariant {
        x: 123_i8,
        y: false,
        z: String::from("value"),
    });
}



// MAPS-LIKE
// ---------
use core::hash::BuildHasherDefault;
use fnv::FnvHasher;
use indexmap::IndexMap;
use indexmap::indexmap_with_default;

type MyMapType = IndexMap<String, i8, BuildHasherDefault<FnvHasher>>;

#[test]
fn deserialize_to_map_key_str() {
    let my_map_value = indexmap_with_default!{
        FnvHasher;
        String::from("key1") => 123_i8,
        String::from("key2") => 45_i8,
    };
    assert_deserialize_value_ok!(&[
    //   [{]
        0x7B,
    //        [u]  [4]   [k]   [e]   [y]   [1]
            0x55, 0x04, 0x6B, 0x65, 0x79, 0x31,   0x69, 0x7B,
    //        [u]  [4]   [k]   [e]   [y]   [2]
            0x55, 0x04, 0x6B, 0x65, 0x79, 0x32,   0x69, 0x2D,
    //   [}]
        0x7D,

    ], MyMapType, my_map_value);
}



//
// TODO fn deserialize_a_large_collection_of_diverse_value_types()
//      to make sure the buffer is completely consumed
//
//





