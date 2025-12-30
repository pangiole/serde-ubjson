// See http://dmitry-ra.github.io/ubjson-test-suite/json-converter.html

use serde_ubj::*;

#[path = "model.rs"]
mod model;

#[path = "text.rs"]
mod text;


macro_rules! assert_serialize_ok {
    ($value:expr, $expected:expr) => {
        let mut buffer: Vec<u8> = Vec::new();
        let result = to_writer(&mut buffer, &$value);
        assert!(result.is_ok());
        assert_eq!(buffer.as_slice(), $expected);
    };
}

macro_rules! assert_serialize_err {
    ($value:expr, $err:expr) => {
        let mut buffer = Vec::new();
        let result = to_writer(&mut buffer, &$value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), $err.to_string());
    };
}

macro_rules! assert_serialize_map_key_err {
    ($key:expr, $key_type:expr) => {
        assert_serialize_err!(
            indexmap_with_default!{
                FnvHasher;
                $key => String::from("value"),
            },
            UbjError::IllegalKeyType($key_type)
        );
    };
}


// ---------------------------------------------------------------------------------
// S C A L A R    values
// ---------------------------------------------------------------------------------


#[test]
fn serialize_unit() {
    assert_serialize_ok!((), &[0x5A]);
}


#[test]
fn serialize_bool() {
    assert_serialize_ok!(true , &[0x54]);
    assert_serialize_ok!(false, &[0x46]);
}

#[test]
fn serialize_i8() {
    // MIN                            MAX
    // -128             0             127
    // |----------------|---------------|
    //
    assert_serialize_ok!(-123_i8, &[0x69, 0x85]);
    assert_serialize_ok!(123_i8 , &[0x69, 0x7B]);
}

#[test]
fn serialize_u8() {
    // MIN                            MAX
    // 0                127           255
    // |----------------|--------------|
    //
    assert_serialize_ok!(123_u8, &[ 0x55, 0x7B ]);
    assert_serialize_ok!(254_u8, &[ 0x55, 0xFE ]);
}

#[test]
fn serialize_i16() {
    // MIN                                                                  MAX
    // -32768                  -128   0   127  255                        32767
    // |---------- . ------------|====|~~~~|~~~~|----------- . ------------|
    //
    assert_serialize_ok!(-32700_i16, &[ 0x49, 0x80, 0x44 ]);
    assert_serialize_ok!(-123_i16  , &[ 0x69, 0x85       ]); // ==> int8
    assert_serialize_ok!(123_i16   , &[ 0x55, 0x7B       ]); // ~~> uint8
    assert_serialize_ok!(254_i16   , &[ 0x55, 0xFE       ]); // ~~> uint8
    assert_serialize_ok!(32700_i16 , &[ 0x49, 0x7F, 0xBC ]);
}


#[test]
fn serialize_u16() {
    // MIN                                                                                  MAX
    // 0   127  255                                  32767                                 65535
    // |~~~~|~~~~|***************** . ****************|----------------- . -----------------|
    //
    assert_serialize_ok!(123_u16  , &[ 0x55, 0x7B                   ]); // ~~> uint8
    assert_serialize_ok!(254_u16  , &[ 0x55, 0xFE                   ]); // ~~> uint8
    assert_serialize_ok!(32700_u16, &[ 0x49, 0x7F, 0xBC             ]); // **> int16
    assert_serialize_ok!(65000_u16, &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8 ]); // ``> int32
}


#[test]
fn serialize_i32() {
    // MIN                                                                                                                      MAX
    // -2147483648             -65538        -32768        -128   0   127   255         32767         65535                2147483647
    // |-------------------------|----- . -----|***** . *****|====|~~~~|~~~~|***** . *****|----- .-----|-------------------------|
    //
    assert_serialize_ok!(-1247483648_i32, &[ 0x6C, 0xB5, 0xA4, 0xE9, 0x00 ]);
    assert_serialize_ok!(-65000_i32     , &[ 0x6C, 0xFF, 0xFF, 0x02, 0x18 ]);
    assert_serialize_ok!(-32700_i32     , &[ 0x49, 0x80, 0x44             ]); // **> int16
    assert_serialize_ok!(-123_i32       , &[ 0x69, 0x85                   ]); // ==> int8
    assert_serialize_ok!(123_i32        , &[ 0x55, 0x7B                   ]); // ~~> uint8
    assert_serialize_ok!(254_i32        , &[ 0x55, 0xFE                   ]); // ~~> uint8
    assert_serialize_ok!(32700_i32      , &[ 0x49, 0x7F, 0xBC             ]); // **> int16
    assert_serialize_ok!(65000_i32      , &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8 ]);
    assert_serialize_ok!(1247483648_i32 , &[ 0x6C, 0x4A, 0x5B, 0x17, 0x00 ]);
}

#[test]
fn serialize_u32() {
    assert_serialize_ok!(123_u32       , &[ 0x55, 0x7B                                           ]); // ~~> uint8
    assert_serialize_ok!(254_u32       , &[ 0x55, 0xFE                                           ]); // ~~> uint8
    assert_serialize_ok!(32700_u32     , &[ 0x49, 0x7F, 0xBC                                     ]); // **> int16
    assert_serialize_ok!(65000_u32     , &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8                         ]); // ``> int32
    assert_serialize_ok!(1247483648_u32, &[ 0x6C, 0x4A, 0x5B, 0x17, 0x00                         ]); // ``> int32
    assert_serialize_ok!(4294967290_u32, &[ 0x4C, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFA ]); //     int64
}

#[test]
fn serialize_i64() {
    assert_serialize_ok!(-922337203685477_i64, &[ 0x4C, 0xFF, 0xFC, 0xB9, 0x23, 0xA2, 0x9C, 0x77, 0x9B ]);
    assert_serialize_ok!(-4294967290_i64     , &[ 0x4C, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x06 ]);
    assert_serialize_ok!(-1247483648_i64     , &[ 0x6C, 0xB5, 0xA4, 0xE9, 0x00                         ]); // ``> int32
    assert_serialize_ok!(-65000_i64          , &[ 0x6C, 0xFF, 0xFF, 0x02, 0x18                         ]); // ``> int32
    assert_serialize_ok!(-32700_i64          , &[ 0x49, 0x80, 0x44                                     ]); // **> int16
    assert_serialize_ok!(-123_i64            , &[ 0x69, 0x85                                           ]); // ==> int8
    assert_serialize_ok!(123_i64             , &[ 0x55, 0x7B                                           ]); // ~~> uint8
    assert_serialize_ok!(254_i64             , &[ 0x55, 0xFE                                           ]); // ~~> uint8
    assert_serialize_ok!(32700_i64           , &[ 0x49, 0x7F, 0xBC                                     ]); // **> int16
    assert_serialize_ok!(65000_i64           , &[ 0x6C, 0x00, 0x00, 0xFD, 0xE8                         ]); // ``> int32
    assert_serialize_ok!(1247483648_i64      , &[ 0x6C, 0x4A, 0x5B, 0x17, 0x00                         ]); // ``> int32
    assert_serialize_ok!(4294967290_i64      , &[ 0x4C, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFA ]);
    assert_serialize_ok!(922337203685477_i64 , &[ 0x4C, 0x00, 0x03, 0x46, 0xDC, 0x5D, 0x63, 0x88, 0x65 ]);
}


#[test]
fn serialize_u64() {
    assert_serialize_ok!(922337203685477_u64, &[0x4C, 0x00, 0x03, 0x46, 0xDC, 0x5D, 0x63, 0x88, 0x65]);
    assert_serialize_err!(u64::MAX, UbjError::Unsupported("Rust u64 values greater than i64::MAX"));
}

#[test]
fn serialize_i128() {
    assert_serialize_err!(i128::MAX, UbjError::Unsupported("Rust i128 values"));
}

#[test]
fn serialize_u128() {
    assert_serialize_err!(u128::MAX, UbjError::Unsupported("Rust u128 values"));
}

#[test]
fn serialize_float_f32() {
    assert_serialize_ok!(
        0.15625_f32,
        &[0x64, 0x3E, 0x20, 0x00, 0x00]
    );
}

#[test]
fn serialize_float_f64() {
    assert_serialize_ok!(
        1.23456789_f64,
        &[0x44, 0x3F, 0xF3, 0xC0, 0xCA, 0x42, 0x83, 0xDE, 0x1B]
    );
}

#[test]
fn serialize_char_ok() {
    assert_serialize_ok!('H', &[0x43, 0x48]);
    assert_serialize_err!('ü', UbjError::CharNotAscii('ü' as u32));
}

#[test]
fn serialize_string_45_bytes_long() {
    let (text, ubj) = text::generate(45);
    assert_eq!(ubj[1], 0x55);
    assert_serialize_ok!(text, ubj.as_slice());
}

#[test]
fn serialize_string_230_bytes_long() {
    let (text, ubj) = text::generate(230);
    assert_eq!(ubj[1], 0x55); // u8
    assert_serialize_ok!(text, ubj.as_slice());
}

#[test]
fn serialize_string_15300_bytes_long() {
    let (text, ubj) = text::generate(15300);
    assert_eq!(ubj[1], 0x49); // i16
    assert_serialize_ok!(text, ubj.as_slice());
}

#[test]
fn serialize_string_7483648_bytes_long() {
    let (text, ubj) = text::generate(7483648);
    assert_eq!(ubj[1], 0x6C); // i32
    assert_serialize_ok!(text, ubj.as_slice());
}

#[test]
#[ignore]
// WARNING this test requires 2.15 GB of RAM (too expensive for CI)
fn serialize_string_2147483648_bytes_long() {
    let (text, ubj) = text::generate(2147483648);
    assert_eq!(ubj[1], 0x4C); // i64
    assert_serialize_ok!(text, ubj.as_slice());
}

// TODO serialize_bytes
// #[test]
// #[ignore]
// fn serialize_bytes() {
//     let my_data: [u8; 3] = [123_u8, 45_u8, 67_u8];
//     let my_bytes_wrapper = model::MyBytesWrapper {
//         bytes: &my_data[..],
//         //byte_buf: my_data.to_vec(),
//         //byte_array: my_data,
//     };
//     assert_serialize_ok!(my_bytes_wrapper, &[
//         0x5B,
//             0x7B, 0x2D, 0x43,
//         0x5D
//     ]);
// }


// ---------------------------------------------------------------------------------
//  C O M P O U N D   values
// ---------------------------------------------------------------------------------

#[test]
fn serialize_none() {
    assert_serialize_ok!(None as Option<()>, &[0x5A]);
}

#[test]
fn serialize_some_bool() {
    assert_serialize_ok!(Some(true) , &[0x54]);
    assert_serialize_ok!(Some(false), &[0x46]);
}

#[test]
fn serialize_some_i8() {
    assert_serialize_ok!(Some(-123_i8), &[0x69, 0x85]);
    assert_serialize_ok!(Some(123_i8 ), &[0x69, 0x7B]);
}


// SEQUENCE-LIKE
// --------
#[test]
fn serialize_vector() {
    let my_sequence = vec![12_i8, 64_i8, 123_i8];
    assert_serialize_ok!(
        my_sequence,
        &[
            0x5B,
                0x69, 0x0C,
                0x69, 0x40,
                0x69, 0x7B,
            0x5D
        ]
    );
}

#[test]
fn serialize_array() {
    let my_array: [i8; 3]  = [12_i8, 64_i8, 123_i8];
    assert_serialize_ok!(
        my_array,
        &[
            0x5B,
                0x69, 0x0C,
                0x69, 0x40,
                0x69, 0x7B,
            0x5D,
        ]
    );
}

#[test]
fn serialize_tuple() {
    let my_tuple: (i8, i8, i8) = (12_i8, 64_i8, 123_i8);
    assert_serialize_ok!(
        my_tuple,
        &[
            0x5B,
                0x69, 0x0C,
                0x69, 0x40,
                0x69, 0x7B,
            0x5D
        ]);
}


// STRUCTS
// -------

#[test]
fn serialize_unit_struct() {
    assert_serialize_ok!(model::MyUnitStruct {}, &[
        0x5A
    ]);
}

#[test]
fn serialize_newtype_struct() {
    let my_newtype_struct = model::MyNewtypeStruct(123_i8);
    assert_serialize_ok!(my_newtype_struct, &[
        0x69, 0x7B,
    ]);
}

#[test]
fn serialize_tuple_struct() {
    let my_tuple_struct = model::MyTupleStruct(123_i8, 32700_i16, 1247483648_i32);
    assert_serialize_ok!(
        my_tuple_struct,
        &[
            0x5B,
                0x69, 0x7B,
                0x49, 0x7F, 0xBC,
                0x6C, 0x4A, 0x5B, 0x17, 0x00,
            0x5D,
        ]
    );
}

#[test]
fn serialize_struct() {
    let my_struct = model::MyFieldsStruct {
        x: 123_i8,
        y: false,
        z: String::from("value"),
    };
    assert_serialize_ok!(
        my_struct,
        &[
            0x7B,
                0x55, 0x01, 0x78,    0x69, 0x7B,
                0x55, 0x01, 0x79,    0x46,
                0x55, 0x01, 0x7A,    0x53, 0x55, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65,
            0x7D,
        ]
    );
}


// ENUMS
// ----------

#[test]
fn serialize_unit_variant() {
    use model::MyEnum;
    assert_serialize_ok!(
        MyEnum::MyUnitVariant,
        &[
            0x7B,
            //   variant identifier
            //   [u]  [13]   [M]   [y]   [U]   [n]   [i]   [t]   [V]   [a]   [r]   [i]   [a]   [n]  [t]
                0x55, 0x0D, 0x4D, 0x79, 0x55, 0x6E, 0x69, 0x74, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
            //   variant data
            //   [Z]
                0x5A,
            0x7D
        ]
    );
}

#[test]
fn serialize_newtype_variant() {
    use model::MyEnum;
    assert_serialize_ok!(
        MyEnum::MyNewtypeVariant(String::from("value")),
        &[
            0x7B,
            //   variant identifier
            //   [u]  [16]   [M]   [y]   [N]   [e]   [w]   [t]   [y]   [p]   [e]   [V]   [a]   [r]   [i]   [a]   [n]   [t]
                0x55, 0x10, 0x4D, 0x79, 0x4E, 0x65, 0x77, 0x74, 0x79, 0x70, 0x65, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x74,
            //   variant data
            //   [S]   [u]   [5]   [v]   [a]   [l]   [u]   [e]
                0x53, 0x55, 0x05, 0x76, 0x61, 0x6C, 0x75, 0x65,
            0x7D
        ]
    );
}

#[test]
fn serialize_tuple_variant() {
    use model::MyEnum;
    assert_serialize_ok!(
        MyEnum::MyTupleVariant(123_i8, 32700_i16, 1247483648_i32),
        &[
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
        ]
    );
}

#[test]
fn serialize_struct_variant() {
    use model::MyEnum;
    assert_serialize_ok!(
        MyEnum::MyStructVariant {
            x: 123_i8,
            y: false,
            z: String::from("value"),
        },
        &[
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
        ]
    );
}


// MAPS-LIKE
// ---------

use indexmap::indexmap_with_default;
use fnv::FnvHasher;

#[test]
fn serialize_map_key_str() {
    assert_serialize_ok!(
        indexmap_with_default!{
            FnvHasher;
            "key1" => 123_i8,
            "key2" => 45_i8,
        },
        &[
        //   [{]
            0x7B,
        //        [u]  [4]   [k]   [e]   [y]   [1]
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x31,   0x69, 0x7B,
        //        [u]  [4]   [k]   [e]   [y]   [2]
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x32,   0x69, 0x2D,
        //   [}]
            0x7D,
        ]
    );
}

#[test]
fn serialize_map_key_err_bool() {
    assert_serialize_map_key_err!(true, "bool");
}
#[test]
fn serialize_map_key_err_i8() {
    assert_serialize_map_key_err!(1_i8, "i8");
}
#[test]
fn serialize_map_key_err_i16() {
    assert_serialize_map_key_err!(1_i16, "i16");
}
#[test]
fn serialize_map_key_err_i32() {
    assert_serialize_map_key_err!(1_i32, "i32");
}
#[test]
fn serialize_map_key_err_i64() {
    assert_serialize_map_key_err!(1_i64, "i64");
}
#[test]
fn serialize_map_key_err_u8() {
    assert_serialize_map_key_err!(1_u8, "u8");
}
#[test]
fn serialize_map_key_err_u16() {
    assert_serialize_map_key_err!(1_u16, "u16");
}
#[test]
fn serialize_map_key_err_u32() {
    assert_serialize_map_key_err!(1_u32, "u32");
}
#[test]
fn serialize_map_key_err_u64() {
    assert_serialize_map_key_err!(1_u64, "u64");
}
// #[test]
// TODO fn serialize_map_key_err_f32() {
//     assert_serialize_map_key_err!(1_f32, "f32");
// }
// #[test]
// TODO fn serialize_map_key_err_f64() {
//     assert_serialize_map_key_err!(1_f64, "f64");
// }
#[test]
fn serialize_map_key_err_char() {
    assert_serialize_map_key_err!('c', "char");
}
#[test]
fn serialize_map_key_err_bytes() {
    let data = [0x00u8, 0x00u8, 0x00u8, 0x00u8];
    let bytes = serde_bytes::Bytes::new(&data[..]);
    let my_map = indexmap_with_default!{
        FnvHasher;
        bytes => String::from("value"),
    };
    assert_serialize_err!(my_map, UbjError::IllegalKeyType("&[u8]"));
}
#[test]
fn serialize_map_key_err_none() {
    assert_serialize_map_key_err!(None as Option<()>, "None");
}
#[test]
fn serialize_map_key_some_str() {
    assert_serialize_ok!(
        indexmap_with_default!{
            FnvHasher;
            Some("key1") => 123_i8,
            Some("key2") => 45_i8,
        },
        &[
        //   [{]
            0x7B,
        //        [u]  [4]   [k]   [e]   [y]   [1]
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x31,   0x69, 0x7B,
        //        [u]  [4]   [k]   [e]   [y]   [2]
                0x55, 0x04, 0x6B, 0x65, 0x79, 0x32,   0x69, 0x2D,
        //   [}]
            0x7D,
        ]
    );
}
#[test]
fn serialize_map_key_err_unit() {
    assert_serialize_map_key_err!((), "()");
}
#[test]
fn serialize_map_key_err_vector() {
    assert_serialize_map_key_err!(vec![1_i8, 2_i8, 3_i8], "seq");
}
#[test]
fn serialize_map_key_err_array() {
    assert_serialize_map_key_err!([1_i8, 2_i8, 3_i8], "tuple");
}
#[test]
fn serialize_map_key_err_tuple() {
    assert_serialize_map_key_err!((1_i8, 2_i8, 3_i8), "tuple");
}

#[test]
fn serialize_map_key_err_unit_struct() {
    use model::MyUnitStruct;
    assert_serialize_map_key_err!(MyUnitStruct {}, "struct");
}
#[test]
fn serialize_map_key_err_newtype_struct() {
    use model::MyNewtypeStruct;
    assert_serialize_map_key_err!(MyNewtypeStruct(1_i8), "struct");
}
#[test]
fn serialize_map_key_err_tuple_struct() {
    use model::MyTupleStruct;
    assert_serialize_map_key_err!(MyTupleStruct(1_i8, 2_i16, 3_i32), "struct");
}
#[test]
fn serialize_map_key_err_fields_struct() {
    use model::MyFieldsStruct;
    assert_serialize_map_key_err!(MyFieldsStruct{ x: 1_i8, y: false, z: String::from("value") }, "struct");
}
#[test]
fn serialize_map_key_err_unit_variant() {
    use model::MyEnum;
    assert_serialize_map_key_err!(MyEnum::MyUnitVariant, "enum");
}
#[test]
fn serialize_map_key_err_newtype_variant() {
    use model::MyEnum;
    assert_serialize_map_key_err!(MyEnum::MyNewtypeVariant(String::from("value")), "enum");
}
#[test]
fn serialize_map_key_err_tuple_variant() {
    use model::MyEnum;
    assert_serialize_map_key_err!(MyEnum::MyTupleVariant(1_i8, 2_i16, 3_i32), "enum");
}
#[test]
fn serialize_map_key_err_struct_variant() {
    use model::MyEnum;
    assert_serialize_map_key_err!(MyEnum::MyStructVariant{ x: 1_i8, y: false, z: String::from("value") }, "enum");
}
