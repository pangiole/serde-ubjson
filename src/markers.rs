
#[repr(u8)]
pub enum UbjMarker {
    Null           = 0x5A,  // Z
    True           = 0x54,  // T
    False          = 0x46,  // F
    Int8           = 0x69,  // i  [-128 .. 0 .. 127]
    Uint8          = 0x55,  // U  [0 .. 255]
    Int16          = 0x49,  // I  [-32768 .. 0 .. 32767]
    Int32          = 0x6C,  // l  [-2147483648 .. 0 .. 2147483647]
    Int64          = 0x4C,  // L  [-9223372036854775808 .. 0 .. 9223372036854775807]
    Float32        = 0x64,  // d
    Float64        = 0x44,  // d
    Char           = 0x43,  // C
    Str            = 0x53,  // S

    OpeningBracket = 0x5B,  // [
    ClosingBracket = 0x5D,  // ]

    OpeningBrace   = 0x7B,  // {
    ClosingBrace   = 0x7D,  // }
}