use crate::inner::IoBufRead;
use crate::inner::err::{UbjError, UbjResult};
use crate::inner::markers::UbjMarker;

pub struct UbjReader<R>
where
    R: IoBufRead,
{
    underlying: R,
    bytes_consumed: usize,
}

impl<R> UbjReader<R>
where
    R: IoBufRead,
{
    /// Creates a new UBJ reader instance that delegates all operations to the provided reader.
    pub fn new(buf_read: R) -> Self {
        Self {
            underlying: buf_read,
            bytes_consumed: 0,
        }
    }

    fn buf_consume(&mut self, n: usize) {
        self.underlying.consume(n);
        self.bytes_consumed += n;
    }

    fn buf_refill(&mut self) -> UbjResult<&[u8]> {
        // TODO Shouldn't we handle the std:io::Error::WouldBlock case?
        let buf = self.underlying.fill_buf().map_err(UbjError::from_io_error)?;
        if buf.is_empty() {
            Err(UbjError::UnexpectedEof)
        } else {
            Ok(buf)
        }
    }

    fn buf_consume_marker(&mut self, marker: UbjMarker) -> UbjResult<()> {
        let buf = self.buf_refill()?;
        if buf[0] == marker as u8 {
            self.buf_consume(1);
            Ok(())
        } else {
            Err(UbjError::UnexpectedMarker(buf[0]))
        }
    }

    fn buf_consume_bytes<const LEN: usize>(&mut self) -> UbjResult<[u8; LEN]> {
        // Create a new joining stack-allocated buffer and a few useful counters
        let mut joining_buffer = [0u8; LEN];
        let mut pending_bytes = LEN;
        let mut bytes_copied = 0;

        // This is the COPY LOOP
        while pending_bytes > 0 {
            // Always refill the source buffer at every iteration
            let source_buffer = self.buf_refill()?;

            // Derive how many bytes can be copied from the current chunk of the source buffer
            let current_chunk = source_buffer.len().min(pending_bytes);

            // Append more bytes to the local buffer
            joining_buffer[bytes_copied..(bytes_copied + current_chunk)]
                .copy_from_slice(&source_buffer[..current_chunk]);

            // Advance the source buffer by the number of bytes we just copied
            self.buf_consume(current_chunk);

            // Update the counters accordingly
            bytes_copied += current_chunk;
            pending_bytes -= current_chunk;
        }

        Ok(joining_buffer)
    }

    #[inline]
    fn buf_consume_usize(&mut self) -> UbjResult<usize> {
        self.read_uint8()
            .map(|n| n as usize)
            .or_else(|_| self.read_int8().map(|n| n as usize))
            .or_else(|_| self.read_int16().map(|n| n as usize))
            .or_else(|_| self.read_int32().map(|n| n as usize))
            .or_else(|_| self.read_int64().map(|n| n as usize))
    }

    fn buf_consume_text(&mut self, len: usize) -> UbjResult<alloc::string::String> {
        // Create a new joining heap-allocated buffer and a few useful counters
        let mut joining_buffer = alloc::string::String::with_capacity(len);
        let mut pending_bytes = len;

        // This is the COPY LOOP
        while pending_bytes > 0 {
            // Always refill the source buffer at every iteration
            let source_buffer = self.buf_refill()?;

            // Derive how many bytes can be copied from the current chunk of the source buffer
            let current_chunk = source_buffer.len().min(pending_bytes);

            // Takes the current chunk of bytes from the source buffer
            // and tries to convert it to a Rust string slice by applying UTF-8 validation.

            let source_slice = &source_buffer[..current_chunk];
            match core::str::from_utf8(source_slice) {
                Ok(valid_str_slice) => {
                    joining_buffer.push_str(valid_str_slice);
                    self.buf_consume(current_chunk);
                }

                Err(err) => {
                    let valid_bytes_count = err.valid_up_to();
                    let invalid_bytes_count = source_slice.len() - valid_bytes_count;
                    let utf8_bytes_count =
                        Utf8Character::bytes_count(&source_slice[valid_bytes_count]);

                    if invalid_bytes_count >= utf8_bytes_count {
                        // It looks like an unrecoverable UTF-8 error
                        return Err(UbjError::Utf8Error(err));
                    } else {
                        // Attempts to handle the error by taking the valid portion of the source buffer
                        let valid_str_slice =
                            core::str::from_utf8(&source_slice[0..valid_bytes_count])?;
                        joining_buffer.push_str(valid_str_slice);
                        self.buf_consume(current_chunk - invalid_bytes_count);

                        // And then it tries to reconstruct the single UTF-8 character whose bytes
                        // got split between the two successive source buffer chunks
                        match utf8_bytes_count {
                            2 => {
                                let utf8_bytes = self.buf_consume_bytes::<2>()?;
                                let utf8_char = core::str::from_utf8(&utf8_bytes[..])?;
                                joining_buffer.push_str(utf8_char);
                            }
                            3 => {
                                let utf8_bytes = self.buf_consume_bytes::<3>()?;
                                let utf8_char = core::str::from_utf8(&utf8_bytes[..])?;
                                joining_buffer.push_str(utf8_char);
                            }
                            4 => {
                                let utf8_bytes = self.buf_consume_bytes::<4>()?;
                                let utf8_char = core::str::from_utf8(&utf8_bytes[..])?;
                                joining_buffer.push_str(utf8_char);
                            }
                            _ => unreachable!(),
                        };

                        pending_bytes -= utf8_bytes_count - invalid_bytes_count;
                    }
                }
            }

            // Update the counters accordingly
            pending_bytes -= current_chunk;
        }
        Ok(joining_buffer)
    }

    // --------------------------------------------------------------------------------------------
    // P U B L I C    m e t h o d s
    //

    pub fn read_bool(&mut self) -> UbjResult<bool> {
        let bytes = self.buf_consume_bytes::<1>()?;
        let marker = bytes[0];
        if marker == UbjMarker::True as u8 {
            Ok(true)
        } else if marker == UbjMarker::False as u8 {
            Ok(false)
        } else {
            Err(UbjError::UnexpectedMarker(marker))
        }
    }

    pub fn read_null(&mut self) -> UbjResult<()> {
        self.buf_consume_marker(UbjMarker::Null)?;
        Ok(())
    }

    pub fn read_uint8(&mut self) -> UbjResult<u8> {
        self.buf_consume_marker(UbjMarker::Uint8)?;
        let bytes = self.buf_consume_bytes::<1>()?;
        Ok(bytes[0])
    }

    pub fn read_int8(&mut self) -> UbjResult<i8> {
        self.buf_consume_marker(UbjMarker::Int8)?;
        let bytes = self.buf_consume_bytes::<1>()?;
        let value = i8::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_int16(&mut self) -> UbjResult<i16> {
        self.buf_consume_marker(UbjMarker::Int16)?;
        let bytes = self.buf_consume_bytes::<2>()?;
        let value = i16::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_int32(&mut self) -> UbjResult<i32> {
        self.buf_consume_marker(UbjMarker::Int32)?;
        let bytes = self.buf_consume_bytes::<4>()?;
        let value = i32::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_int64(&mut self) -> UbjResult<i64> {
        self.buf_consume_marker(UbjMarker::Int64)?;
        let bytes = self.buf_consume_bytes::<8>()?;
        let value = i64::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_float32(&mut self) -> UbjResult<f32> {
        self.buf_consume_marker(UbjMarker::Float32)?;
        let bytes = self.buf_consume_bytes::<4>()?;
        let value = f32::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_float64(&mut self) -> UbjResult<f64> {
        self.buf_consume_marker(UbjMarker::Float64)?;
        let bytes = self.buf_consume_bytes::<8>()?;
        let value = f64::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_char(&mut self) -> UbjResult<char> {
        // NOTE
        // The Universal Binary JSON format specifies "char" to be an unsigned integer byte
        // meant to represent a single ASCII character whose position lays within the
        // 0..127 range of the Unicode standard table.
        //
        self.buf_consume_marker(UbjMarker::Char)?;
        let bytes = self.buf_consume_bytes::<1>()?;
        let c = bytes[0];
        if c.is_ascii() {
            Ok(c as char)
        } else {
            Err(UbjError::CharNotAscii(c as u32))
        }
    }

    pub fn read_marked_string(&mut self) -> UbjResult<alloc::string::String> {
        self.buf_consume_marker(UbjMarker::String)
            .and_then(|_| self.read_unmarked_string())
    }

    pub fn read_unmarked_string(&mut self) -> UbjResult<alloc::string::String> {
        let len = self.buf_consume_usize()?;
        self.buf_consume_text(len)
    }

    // TODO pub fn read_str(&mut self) -> UbjResult<&str> {
    //     let str_len = self.buf_consume_usize()?;
    //
    //     let buf = self.buf_refill()?;
    //     if buf.len() < str_len {
    //         return Err(UbjError::BufferTooSmall(
    //             self.bytes_consumed + str_len + str_len.to_be_bytes().len(),
    //         ));
    //     }
    //
    //     let source_slice = &buf[..str_len];
    //     let str_slice = core::str::from_utf8(source_slice)?;
    //     Ok(str_slice)
    // }

    pub fn read_start_array(&mut self) -> UbjResult<()> {
        self.buf_consume_marker(UbjMarker::StartArray)
    }

    pub fn read_end_array(&mut self) -> UbjResult<()> {
        self.buf_consume_marker(UbjMarker::EndArray)
    }

    pub fn read_start_object(&mut self) -> UbjResult<()> {
        self.buf_consume_marker(UbjMarker::StartObject)
    }

    pub fn read_end_object(&mut self) -> UbjResult<()> {
        self.buf_consume_marker(UbjMarker::EndObject)
    }
}

struct Utf8Character;

impl Utf8Character {
    /// Discover the bytes count of a UTF-8 character by inspect the leading bits of the first byte.
    //
    // UTF-8 is a "prefix code" meaning the first byte tells exactly how many bytes follow it.
    // Our solution doesn't need to look at the whole first byte; it just looks at how many 1s appear
    // before the first 0 (starting from the most significant bit).
    //
    // In Rust, we can use bitwise operators to check these patterns. For example, to check for a
    // 4-bytes UTF-8 character (starting with 0xF0), we would check if the first five bits are 11110.
    //
    fn bytes_count(first_byte: &u8) -> usize {
        if first_byte & 0b1000_0000 == 0 {
            1 // 0xxxxxxx
        } else if first_byte & 0b1110_0000 == 0b1100_0000 {
            2 // 110xxxxx
        } else if first_byte & 0b1111_0000 == 0b1110_0000 {
            3 // 1110xxxx
        } else if first_byte & 0b1111_1000 == 0b1111_0000 {
            4 // 11110xxx
        } else {
            0 // Invalid UTF-8 prefix!
        }
    }
}
