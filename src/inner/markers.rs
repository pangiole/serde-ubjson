
#[repr(u8)]
pub enum UbjMarker {
    Null         = 0x5A,  // Z
    True         = 0x54,  // T
    False        = 0x46,  // F
    Int8         = 0x69,  // i  [-128 .. 0 .. 127]
    Uint8        = 0x55,  // U  [0 .. 255]
    Int16        = 0x49,  // I  [-32768 .. 0 .. 32767]
    Int32        = 0x6C,  // l  [-2147483648 .. 0 .. 2147483647]
    Int64        = 0x4C,  // L  [-9223372036854775808 .. 0 .. 9223372036854775807]
    Float32      = 0x64,  // d
    Float64      = 0x44,  // D
    Char         = 0x43,  // C
    String       = 0x53,  // S

    StartArray   = 0x5B,  // [
    EndArray     = 0x5D,  // ]

    StartObject  = 0x7B,  // {
    EndObject    = 0x7D,  // }
}