// See http://dmitry-ra.github.io/ubjson-test-suite/json-converter.html
use serde_ubj::*;

#[test]
fn serialize_bool() {
    assert_serialize_ok(true , &[0x54]);
    assert_serialize_ok(false, &[0x46]);
}

#[test]
fn serialize_i8() {
    // MIN                            MAX
    // -128             0             127
    // |----------------|---------------|
    //
    assert_serialize_ok(-123_i8, &[0x69, 0x85]);
    assert_serialize_ok(123_i8 , &[0x69, 0x7B]);
}

#[test]
fn serialize_u8() {
    // MIN                            MAX
    // 0                127           255
    // |----------------|--------------|
    //
    assert_serialize_ok(123_u8, &[ 0x55, 0x7B ]);
    assert_serialize_ok(254_u8, &[ 0x55, 0xFE ]);
}

#[test]
fn serialize_i16() {
    // MIN                                                                  MAX
    // -32768                  -128   0   127  255                        32767
    // |---------- . ------------|====|~~~~|~~~~|----------- . ------------|
    //
    assert_serialize_ok(-32700_i16, &[ 0x49, 0x80, 0x44 ]);
    assert_serialize_ok(-123_i16  , &[ 0x69, 0x85       ]); // ==> int8
    assert_serialize_ok(123_i16   , &[ 0x55, 0x7B       ]); // ~~> uint8
    assert_serialize_ok(254_i16   , &[ 0x55, 0xFE       ]); // ~~> uint8
    assert_serialize_ok(32700_i16 , &[ 0x49, 0x7F, 0xBC ]);
}


#[test]
fn serialize_u16() {
    // MIN                                                                                  MAX
    // 0   127  255                                  32767                                 65535
    // |~~~~|~~~~|***************** . ****************|----------------- . -----------------|
    //
    assert_serialize_ok(123_u16  , &[ 0x55, 0x7B                   ]); // ~~> uint8
    assert_serialize_ok(254_u16  , &[ 0x55, 0xFE                   ]); // ~~> uint8
    assert_serialize_ok(32700_u16, &[ 0x49, 0x7F, 0xBC             ]); // **> int16
    assert_serialize_ok(65000_u16, &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8 ]); // ``> int32
}


#[test]
fn serialize_i32() {
    // MIN                                                                                                                      MAX
    // -2147483648             -65538        -32768        -128   0   127   255         32767         65535                2147483647
    // |-------------------------|----- . -----|***** . *****|====|~~~~|~~~~|***** . *****|----- .-----|-------------------------|
    //
    assert_serialize_ok(-1247483648_i32, &[ 0x6C, 0xB5, 0xA4, 0xE9, 0x00 ]);
    assert_serialize_ok(-65000_i32     , &[ 0x6C, 0xFF, 0xFF, 0x02, 0x18 ]);
    assert_serialize_ok(-32700_i32     , &[ 0x49, 0x80, 0x44             ]); // **> int16
    assert_serialize_ok(-123_i32       , &[ 0x69, 0x85                   ]); // ==> int8
    assert_serialize_ok(123_i32        , &[ 0x55, 0x7B                   ]); // ~~> uint8
    assert_serialize_ok(254_i32        , &[ 0x55, 0xFE                   ]); // ~~> uint8
    assert_serialize_ok(32700_i32      , &[ 0x49, 0x7F, 0xBC             ]); // **> int16
    assert_serialize_ok(65000_i32      , &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8 ]);
    assert_serialize_ok(1247483648_i32 , &[ 0x6C, 0x4A, 0x5B, 0x17, 0x00 ]);
}

#[test]
fn serialize_u32() {
    assert_serialize_ok(123_u32       , &[ 0x55, 0x7B                                           ]); // ~~> uint8
    assert_serialize_ok(254_u32       , &[ 0x55, 0xFE                                           ]); // ~~> uint8
    assert_serialize_ok(32700_u32     , &[ 0x49, 0x7F, 0xBC                                     ]); // **> int16
    assert_serialize_ok(65000_u32     , &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8                         ]); // ``> int32
    assert_serialize_ok(1247483648_u32, &[ 0x6C, 0x4A, 0x5B, 0x17, 0x00                         ]); // ``> int32
    assert_serialize_ok(4294967290_u32, &[ 0x4C, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFA ]); //     int64
}

#[test]
fn serialize_i64() {
    assert_serialize_ok(-922337203685477_i64, &[ 0x4C, 0xFF, 0xFC, 0xB9, 0x23, 0xA2, 0x9C, 0x77, 0x9B ]);
    assert_serialize_ok(-4294967290_i64     , &[ 0x4C, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x06 ]);
    assert_serialize_ok(-1247483648_i64     , &[ 0x6C, 0xB5, 0xA4, 0xE9, 0x00                         ]); // ``> int32
    assert_serialize_ok(-65000_i64          , &[ 0x6C, 0xFF, 0xFF, 0x02, 0x18                         ]); // ``> int32
    assert_serialize_ok(-32700_i64          , &[ 0x49, 0x80, 0x44                                     ]); // **> int16
    assert_serialize_ok(-123_i64            , &[ 0x69, 0x85                                           ]); // ==> int8
    assert_serialize_ok(123_i64             , &[ 0x55, 0x7B                                           ]); // ~~> uint8
    assert_serialize_ok(254_i64             , &[ 0x55, 0xFE                                           ]); // ~~> uint8
    assert_serialize_ok(32700_i64           , &[ 0x49, 0x7F, 0xBC                                     ]); // **> int16
    assert_serialize_ok(65000_i64           , &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8                         ]); // ``> int32
    assert_serialize_ok(1247483648_i64      , &[ 0x6C, 0x4A, 0x5B, 0x17, 0x00                         ]); // ``> int32
    assert_serialize_ok(4294967290_i64      , &[ 0x4C, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFA ]);
    assert_serialize_ok(922337203685477_i64 , &[ 0x4C, 0x00, 0x03, 0x46, 0xDC, 0x5D, 0x63, 0x88, 0x65 ]);
}

#[test]
#[ignore]
fn serialize_huge() {
    todo!()
}

#[test]
fn serialize_u64() {
    assert_serialize_ok(922337203685477_u64, &[0x4C, 0x00, 0x03, 0x46, 0xDC, 0x5D, 0x63, 0x88, 0x65]);
    assert_serialize_err(u64::MAX, serde_ubj::UbjError::UnimplementedValueType("u64"));
}

#[test]
fn serialize_float_f32() {
    assert_serialize_ok(0.15625_f32, &[0x64, 0x3E, 0x20, 0x00, 0x00]);
}

#[test]
fn serialize_float_f64() {
    assert_serialize_ok(
        16777216.125_f64,
        &[0x44, 0x41, 0x70, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00],
    );
}

// TODO Test when serializing a char that happend to be out of the 0..127 range
#[test]
fn serialize_char_ok() {
    assert_serialize_ok('H', &[0x43, 0x48]);
}

#[test]
fn serialize_char_out_of_range() {
    assert_serialize_err('ü', UbjError::IllegalChar('ü'));
}


#[test]
fn serialize_str_45_long() {
    let text = generate_multilingual_text(45);
    let expected = build_expected_bytes(text.as_bytes());
    assert_eq!(expected[1], 0x55);
    assert_serialize_ok(text.as_str(), expected.as_slice());
}

#[test]
fn serialize_str_230_long() {
    let text = generate_multilingual_text(230);
    let expected = build_expected_bytes(text.as_bytes());
    assert_eq!(expected[1], 0x55); // u8
    assert_serialize_ok(text.as_str(), expected.as_slice());
}

#[test]
fn serialize_str_15300_long() {
    let s = generate_multilingual_text(15300);
    let expected = build_expected_bytes(s.as_bytes());
    assert_eq!(expected[1], 0x49); // i16
    assert_serialize_ok(s.as_str(), expected.as_slice());
}

#[test]
fn serialize_str_7483648_long() {
    let text = generate_multilingual_text(7483648);
    let expected = build_expected_bytes(text.as_bytes());
    assert_eq!(expected[1], 0x6C); // i32
    assert_serialize_ok(text.as_str(), expected.as_slice());
}

#[test]
#[ignore]
// WARNING this test requires 2.15 GB of RAM (too expensive for CI)
fn serialize_str_2147483648_long() {
    let text = generate_multilingual_text(2147483648);
    let expected = build_expected_bytes(text.as_bytes());
    assert_eq!(expected[1], 0x4C); // i64
    assert_serialize_ok(text.as_str(), expected.as_slice());
}

#[test]
fn serialize_unit() {
    assert_serialize_ok((), &[0x5A]);
}

#[test]
fn serialize_none() {
    assert_serialize_ok(None as Option<()>, &[0x5A]);
}

// By default, standard Rust types like Vec<u8> or &[u8] do not trigger serialize_bytes().
// Instead, they are treated as generic sequences and invoke serialize_seq(), which processes
// each byte individually as a u8 (see https://docs.rs/serde/1.0.228/serde/trait.Serializer.html#tymethod.serialize_bytes)
//
// serialize_bytes() is invoked only when a type explicitly calls it within its serde::Serialize
// implementation. This happens when using the serde_bytes crate, and when the data is wrapped
// in serde_bytes::Bytes (for &[u8]) or serde_bytes::ByteBuf (for Vec<u8>)
//

#[derive(serde::Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    //#[serde(with = "serde_bytes")]
    //byte_buf: Vec<u8>,

    //#[serde(with = "serde_bytes")]
    //byte_array: [u8; 3],
}

#[test]
#[ignore]
fn serialize_bytes() {
    let data: [u8; 3] = [123_u8, 45_u8, 67_u8];
    let value = Efficient {
        bytes: &data[..],
        //byte_buf: data.to_vec(),
        //byte_array: data,
    };
    assert_serialize_ok(value, &[
        0x5B,
            0x7B, 0x2D, 0x43,
        0x5D
    ]);
}

// TODO Add more test cases for most of the std (standard) types, such as std::time::Instant and similar


// ---------------------------------------------------------------------------------

#[test]
fn serialize_some() {
    assert_serialize_ok(Some(true), &[0x54]);
}

#[derive(serde::Serialize, PartialEq, PartialOrd, Eq, Ord)]
struct UnitStruct;

#[test]
fn serialize_unit_struct() {
    assert_serialize_ok(UnitStruct {}, &[0x7B, 0x7D]);
}

#[derive(serde::Serialize, PartialEq, PartialOrd, Eq, Ord)]
enum Enum1 {
    UnitVariant,
}

#[test]
fn serialize_unit_variant() {
    assert_serialize_ok(
        Enum1::UnitVariant,
        &[
            0x53, 0x55, 0x0B, 0x55, 0x6E, 0x69, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74
        ],
    );
}

#[derive(serde::Serialize, PartialEq, PartialOrd, Eq, Ord)]
struct NewtypeStruct(i8);

#[test]
fn serialize_newtype_struct() {
    assert_serialize_ok(NewtypeStruct(123_i8), &[0x69, 0x7B]);
}

#[derive(serde::Serialize)]
enum Enum2 {
    NewtypeVariant(&'static str),
}
#[test]
fn serialize_newtype_variant() {
    assert_serialize_ok(
        Enum2::NewtypeVariant("value1"),
        &[
            0x7B,

                0x55, 0x0E, 0x4E, 0x65, 0x77, 0x74, 0x79, 0x70, 0x65, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
                0x53, 0x55, 0x06, 0x76, 0x61, 0x6C, 0x75, 0x65, 0x31,
            0x7D,
        ],
    );
}

#[test]
fn serialize_sequence() {
    assert_serialize_ok(
        vec![12_i8, 64_i8],
        &[
            0x5B,
                0x69, 0x0C,
                0x69, 0x40,
            0x5D
        ]
    );
}

#[test]
fn serialize_tuple() {
    let fixed_size_array: [u8; 2]  = [123_u8, 254_u8];
    assert_serialize_ok(
        fixed_size_array,
        &[
            0x5B,
                0x55, 0x7B,
                0x55, 0xFE,
            0x5D,
        ]
    );

    let fixed_size_tuple: (i8, i8) = (12_i8, 64_i8);
    assert_serialize_ok(
        fixed_size_tuple,
        &[
            0x5B,
                0x69, 0x0C,
                0x69, 0x40,
            0x5D
        ]);
}

#[derive(serde::Serialize, PartialEq, PartialOrd, Eq, Ord)]
struct TupleStruct(i8, i16, i32);

#[test]
fn serialize_tuple_struct() {
    assert_serialize_ok(
        TupleStruct(123_i8, 32700_i16, 1247483648_i32),
        &[
            0x5B,
                0x69, 0x7B,
                0x49, 0x7F, 0xBC,
                0x6C, 0x4A, 0x5B, 0x17, 0x00,
            0x5D,
        ],
    );
}

#[derive(serde::Serialize)]
enum Enum3 {
    TupleVariant(i8, i16, i32),
}

#[test]
fn serialize_tuple_variant() {
    assert_serialize_ok(
        Enum3::TupleVariant(123_i8, 32700_i16, 1247483648_i32),
        &[
            0x5B,
                0x69, 0x7B,
                0x49, 0x7F, 0xBC,
                0x6C, 0x4A, 0x5B, 0x17, 0x00,
            0x5D,
        ],
    )
}

#[derive(serde::Serialize)]
enum Enum4 {
    StructVariant { x: i8, y: bool },
}

#[test]
fn serialize_struct_variant() {
    assert_serialize_ok(
        Enum4::StructVariant {
            x: 123_i8,
            y: false,
        },
        &[
            0x7B,
                0x55, 0x0D, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
                0x7B,
                    0x55, 0x01, 0x78, 0x69, 0x7B,
                    0x55, 0x01, 0x79, 0x46,
                0x7D,
            0x7D,
        ],
    );
}

#[derive(serde::Serialize, PartialEq, PartialOrd, Eq, Ord)]
struct Struct {
    x: i8,
    y: bool,
}

#[test]
fn serialize_struct() {
    assert_serialize_ok(
        Struct {
            x: 123_i8,
            y: false,
        },
        &[
            0x7B,
                0x55, 0x01, 0x78, 0x69, 0x7B,
                0x55, 0x01, 0x79, 0x46,
            0x7D,
        ],
    );
}


// TODO How about testing no_std builds
use std::collections::BTreeMap as Map;

#[test]
fn serialize_map_key_str() {
    let mut map = Map::new();
    map.insert("key1", 123_i8);
    map.insert("key2", 45_i8);

    assert_serialize_ok(
        map,
        &[
            0x7B, // {
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x31,   0x69, 0x7B,   // "key1": 123
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x32,   0x69, 0x2D,   // "key2": 45
            0x7D, // }
        ],
    );
}

#[test]
fn serialize_map_key_bool() {
    let mut map = Map::new();
    map.insert(true, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("bool"));
}

#[test]
fn serialize_map_key_i8() {
    let mut map = Map::new();
    map.insert(1_i8, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("i8"));
}

#[test]
fn serialize_map_key_i16() {
    let mut map = Map::new();
    map.insert(1_i16, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("i16"));
}

#[test]
fn serialize_map_key_i32() {
    let mut map = Map::new();
    map.insert(1_i32, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("i32"));
}

#[test]
fn serialize_map_key_i64() {
    let mut map = Map::new();
    map.insert(1_i64, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("i64"));
}

#[test]
fn serialize_map_key_u8() {
    let mut map = Map::new();
    map.insert(1_u8, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("u8"));
}

#[test]
fn serialize_map_key_u16() {
    let mut map = Map::new();
    map.insert(1_u16, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("u16"));
}

#[test]
fn serialize_map_key_u32() {
    let mut map = Map::new();
    map.insert(1_u32, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("u32"));
}

#[test]
fn serialize_map_key_u64() {
    let mut map = Map::new();
    map.insert(1_u64, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("u64"));
}

#[test]
#[ignore]
fn serialize_map_key_f32() {
    todo!()
}

#[test]
#[ignore]
fn serialize_map_key_f64() {
    todo!()
}

#[test]
fn serialize_map_key_char() {
    let mut map = Map::new();
    map.insert('c', "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("char"));
}


#[test]
fn serialize_map_key_bytes() {
    let mut map = Map::new();
    let data = [0x00u8, 0x00u8, 0x00u8, 0x00u8];
    let bytes = serde_bytes::Bytes::new(&data[..]);
    map.insert(bytes, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("&[u8]"));
}

#[test]
fn serialize_map_key_none() {
    let mut map = Map::new();
    map.insert(None as Option<()>, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("None"));
}

#[test]
fn serialize_map_key_some() {
    let mut map = Map::new();
    map.insert(Some("key1"), 123_i8);
    map.insert(Some("key2"), 45_i8);

    assert_serialize_ok(
        map,
        &[
            0x7B, // {
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x31,   0x69, 0x7B, // "key1": 123
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x32,   0x69, 0x2D, // "key2": 45
            0x7D, // }
        ],
    );
}

#[test]
fn serialize_map_key_unit() {
    let mut map = Map::new();
    map.insert((), "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("()"));
}

#[test]
fn serialize_map_key_unit_struct() {
    let mut map = Map::new();
    map.insert(UnitStruct {}, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("() struct"));
}

#[test]
fn serialize_map_key_unit_variant() {
    let mut map = Map::new();
    map.insert(Enum1::UnitVariant, 123_i8);
    assert_serialize_ok(
        map,
        &[
            0x7B, // {
                0x55, 0x0B, 0x55, 0x6E, 0x69, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
                0x69, 0x7B,
            0x7D, // }
        ],
    );
}

#[test]
#[ignore]
fn serialize_map_key_newtype_struct() {
    todo!()
    // let mut map = Map::new();
    // map.insert(NewtypeStruct(123_i8), "hello");
    // assert_ok(map, &[
    //     ???
    // ]);
}

#[test]
#[ignore]
fn serialize_map_key_newtype_variant() {
    todo!()
}

#[test]
fn serialize_map_key_seq() {
    let mut map = Map::new();
    map.insert(vec![12_i8, 64_i8], "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("seq"));
}

#[test]
fn serialize_map_key_tuple() {
    let mut map = Map::new();
    map.insert([123_u8, 254_u8], "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("tuple"));
}

#[test]
fn serialize_map_key_tuple_struct() {
    let mut map = Map::new();
    map.insert(TupleStruct(123_i8, 32700_i16, 1247483648_i32), "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("TupleStruct"));
}


#[test]
#[ignore]
fn serialize_map_key_tuple_variant() {
    todo!()
}


#[test]
#[ignore]
fn serialize_map_key_struct_variant() {
    todo!()
}


#[test]
fn serialize_map_key_mp() {
    let mut map = Map::new();
    let mut key = Map::new();
    key.insert("key1", 123_i8);
    map.insert(key, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("map"));
}

#[test]
fn serialize_map_key_struct() {
    let mut map = Map::new();
    map.insert(Struct {x: 123_i8, y: false}, "value");
    assert_serialize_err(map, UbjError::IllegalKeyType("Struct"));
}




// ================================================================================================
//  TEST  HELPERS

/// Generates a multilingual text string that, when UTF-8 encoded, has exactly `byte_count` bytes.
/// Uses a mix of ASCII (1 byte), Latin Extended (2 bytes), CJK (3 bytes), and Emoji (4 bytes).
fn generate_multilingual_text(byte_count: usize) -> String {
    let mut result = String::new();
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

        result.push(ch);
        bytes_added += ch.len_utf8();
        idx += 1;
    }

    result
}

fn build_expected_bytes(bytes: &[u8]) -> Vec<u8> {
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

fn assert_serialize_ok<T>(value: T, expected: &[u8]) where T: serde::Serialize
{
    let mut buffer = Vec::new();
    let result = serde_ubj::to_writer(&mut buffer, &value);
    assert!(result.is_ok());
    assert_eq!(buffer.as_slice(), expected);
}

fn assert_serialize_err<T>(value: T, err: serde_ubj::UbjError) where T: serde::Serialize {
    let mut buffer = Vec::new();
    let result = serde_ubj::to_writer(&mut buffer, &value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), err.to_string());
}

