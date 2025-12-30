use crate::inner::IoWrite;
use crate::inner::err::{UbjError, UbjResult};
use crate::inner::markers::UbjMarker;

pub struct UbjWriter<W>
where
    W: IoWrite,
{
    underlying: W,
}

impl<W> UbjWriter<W>
where
    W: IoWrite,
{
    /// Creates a new UBJ writer instance that delegates all operations to the provided writer.
    pub fn new(writer: W) -> Self {
        Self { underlying: writer }
    }

    /// Flushes the underlying writer.
    pub fn flush(&mut self) -> UbjResult<()> {
        self.underlying.flush().map_err(UbjError::from_io_error)
    }

    // ---------------------------------------------------------------------------------
    //  S C A L A R   values
    // ---------------------------------------------------------------------------------

    pub fn write_null(&mut self) -> UbjResult<()> {
        self.write_marker(UbjMarker::Null)
    }

    pub fn write_bool(&mut self, v: bool) -> Result<(), UbjError> {
        self.write_marker(if v { UbjMarker::True } else { UbjMarker::False })
    }

    pub fn write_int8(&mut self, v: i8) -> UbjResult<()> {
        self.write_marker_and_payload(UbjMarker::Int8, &v.to_be_bytes())
    }

    pub fn write_uint8(&mut self, v: u8) -> UbjResult<()> {
        // Note that there's no benefit in attempting a conversion to int8
        // as we would end up writing 1 single byte anyway
        self.write_marker_and_payload(UbjMarker::Uint8, &v.to_be_bytes())
    }

    pub fn write_int16(&mut self, v: i16) -> UbjResult<()> {
        if v >= i8::MIN as i16 && v < 0 {
            self.write_int8(v as i8)
        } else if v >= 0 && v <= u8::MAX as i16 {
            self.write_uint8(v as u8)
        } else {
            self.write_marker_and_payload(UbjMarker::Int16, &v.to_be_bytes())
        }
    }

    pub fn write_int32(&mut self, v: i32) -> UbjResult<()> {
        if v >= i16::MIN as i32 && v <= i16::MAX as i32 {
            self.write_int16(v as i16)
        } else {
            self.write_marker_and_payload(UbjMarker::Int32, &v.to_be_bytes())
        }
    }

    pub fn write_int64(&mut self, v: i64) -> UbjResult<()> {
        if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
            self.write_int32(v as i32)
        } else {
            self.write_marker_and_payload(UbjMarker::Int64, &v.to_be_bytes())
        }
    }

    pub fn write_float32(&mut self, v: f32) -> UbjResult<()> {
        self.write_marker_and_payload(UbjMarker::Float32, &v.to_be_bytes())
    }

    pub fn write_float64(&mut self, v: f64) -> UbjResult<()> {
        self.write_marker_and_payload(UbjMarker::Float64, &v.to_be_bytes())
    }

    pub fn write_char(&mut self, v: char) -> UbjResult<()> {
        // In Rust, a 'char' type represents a Unicode Scalar Value (4 bytes), while UTF-8 is a
        // variable-width encoding (from 1 to a maximum of 4 bytes). Our goal is to get the integer
        // value representing the character's position in the Unicode standard table (which
        // extends the ASCII standard table beyond the first 0-127 range).
        let utf8_code = u32::from(v);

        // This conversion has not to be confused with turning the given character into the corresponding
        // UTF-8 encoded, variable width, sequence of bytes which, instead, we would get as follows:
        //
        //      let utf8_bytes = c.encode_utf8(&mut [0; 4]);
        //
        // Now, according to the official specification, the char type in Universal Binary JSON is an unsigned integer
        // byte meant to represent a single ASCII character whose position lays within the 0..127 range
        // of the Unicode standard table.
        //
        // See https://github.com/ubjson/universal-binary-json/issues/56
        if utf8_code > 127 {
            Err(UbjError::CharNotAscii(utf8_code))
        } else {
            self.write_marker_and_payload(UbjMarker::Char, &[utf8_code as u8])
        }
    }

    pub fn write_unmarked_string(&mut self, v: &str) -> UbjResult<()> {
        let payload = v.as_bytes();
        let len = payload.len();
        if len <= (i64::MAX as usize) {
            self.write_int64(len as i64)
                .and_then(|_| self.write_payload(payload))
        } else {
            Err(UbjError::Unsupported(
                "Rust String values with length greater than i64::MAX",
            ))
        }
    }

    pub fn write_marked_string(&mut self, v: &str) -> UbjResult<()> {
        self.write_marker(UbjMarker::String)
            .and_then(|_| self.write_unmarked_string(v))
    }

    // ---------------------------------------------------------------------------------
    //  C O M P O U N D   values
    // ---------------------------------------------------------------------------------

    pub fn write_start_array(&mut self) -> UbjResult<()> {
        self.write_marker(UbjMarker::StartArray)
    }

    pub fn write_end_array(&mut self) -> UbjResult<()> {
        self.write_marker(UbjMarker::EndArray)
    }

    pub fn write_start_object(&mut self) -> UbjResult<()> {
        self.write_marker(UbjMarker::StartObject)
    }

    pub fn write_end_object(&mut self) -> UbjResult<()> {
        self.write_marker(UbjMarker::EndObject)
    }


    // PRIVATE methods
    // --------------------------

    fn write_marker(&mut self, marker: UbjMarker) -> UbjResult<()> {
        self.underlying
            .write_all(&[marker as u8])
            .map_err(UbjError::from_io_error)
    }

    fn write_payload(&mut self, payload: &[u8]) -> UbjResult<()> {
        self.underlying
            .write_all(payload)
            .map_err(UbjError::from_io_error)
    }

    fn write_marker_and_payload(&mut self, marker: UbjMarker, payload: &[u8]) -> UbjResult<()> {
        self.write_marker(marker)
            .map(|_| self)
            .and_then(|s| s.write_payload(payload))
    }
}
