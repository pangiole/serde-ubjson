use crate::err::UbjError;
use crate::markers::UbjMarker;
use crate::prelude::*;

pub struct UbjSerializer<'a, W>
where
    W: IoWrite
{
    writer: &'a mut W,
}

impl<'a, W> UbjSerializer<'a, W>
where
    W: IoWrite,
{
    pub fn new(writer: &'a mut W) -> Self {
        UbjSerializer { writer }
    }

    fn flush(&mut self) -> Result<(), UbjError> {
        self.writer.flush().map_err(UbjError::IO)
    }

    fn write_marker(&mut self, marker: UbjMarker) -> Result<(), UbjError> {
        self.writer.write_all(&[marker as u8]).map_err(UbjError::IO)
    }

    fn write_payload(&mut self, payload: &[u8]) -> Result<(), UbjError> {
        self.writer.write_all(payload).map_err(UbjError::IO)
    }

    fn write_marker_and_payload(
        &mut self,
        marker: UbjMarker,
        payload: &[u8],
    ) -> Result<(), UbjError> {
        self.write_marker(marker)
            .map(|_| self)
            .and_then(|s| s.write_payload(payload))
    }

    fn write_bool(&mut self, v: bool) -> Result<(), UbjError> {
        self.write_marker(if v { UbjMarker::True } else { UbjMarker::False })
    }

    fn write_i8(&mut self, v: i8) -> Result<(), UbjError> {
        self.write_marker_and_payload(UbjMarker::Int8, &v.to_be_bytes())
    }

    fn write_u8(&mut self, v: u8) -> Result<(), UbjError> {
        // Note that there's no benefit in attempting a conversion to Universal Binary JSON int8
        // as we would end up writing 1 single byte anyway
        //
        self.write_marker_and_payload(UbjMarker::Uint8, &v.to_be_bytes())
    }

    fn write_i16(&mut self, v: i16) -> Result<(), UbjError> {
        if v >= i8::MIN as i16 && v < 0 {
            return self.write_i8(v as i8);
        } else if v >= 0 && v <= u8::MAX as i16 {
            return self.write_u8(v as u8);
        }
        self.write_marker_and_payload(UbjMarker::Int16, &v.to_be_bytes())
    }

    fn write_i32(&mut self, v: i32) -> Result<(), UbjError> {
        if v >= i16::MIN as i32 && v <= i16::MAX as i32 {
            return self.write_i16(v as i16);
        }
        self.write_marker_and_payload(UbjMarker::Int32, &v.to_be_bytes())
    }

    fn write_i64(&mut self, v: i64) -> Result<(), UbjError> {
        if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
            return self.write_i32(v as i32);
        }
        self.write_marker_and_payload(UbjMarker::Int64, &v.to_be_bytes())
    }

    fn write_f32(&mut self, v: f32) -> Result<(), UbjError> {
        self.write_marker_and_payload(UbjMarker::Float32, &v.to_be_bytes())
    }

    fn write_f64(&mut self, v: f64) -> Result<(), UbjError> {
        self.write_marker_and_payload(UbjMarker::Float64, &v.to_be_bytes())
    }

    fn write_char(&mut self, v: char) -> Result<(), UbjError> {
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
            return Err(UbjError::IllegalChar(v));
        }
        self.write_marker_and_payload(UbjMarker::Char, &[utf8_code as u8])
    }

    fn write_str(&mut self, v: &str) -> Result<(), UbjError> {
        self.write_marker(UbjMarker::Str)?;
        self.write_size_and_utf8_slice(v)
    }

    fn write_none(&mut self) -> Result<(), UbjError> {
        self.write_marker(UbjMarker::Null)
    }

    // TODO fn write_huge(&mut self, number: u64) -> Result<(), UbjError> {

    fn write_size_and_utf8_slice(&mut self, v: &str) -> Result<(), UbjError> {
        // In Rust, a string slice is guaranteed to be a valid UTF-8 encoded sequence of bytes
        let payload = v.as_bytes();
        let len = payload.len();
        if len <= (i64::MAX as usize) {
            self.write_i64(len as i64)?;
        } else {
            return Err(UbjError::UnimplementedValueType(
                "alloc::string::String with len > i64::MAX",
            ));
        }
        self.write_payload(payload)?;
        Ok(())
    }
}

impl<'a, W> serde::Serializer for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    // ---------------------------------------------------------------------------------
    // SIMPLE values
    // ---------------------------------------------------------------------------------

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_i8(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_i16(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_i32(v as i32)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_i32(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_i64(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v <= i64::MAX as u64 {
            return self.write_i64(v as i64);
        }
        Err(UbjError::UnimplementedValueType("u64"))
    }

    // TODO fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {}
    // TODO fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {}

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.write_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_str(v)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.write_none()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    // ---------------------------------------------------------------------------------
    // COMPLEX values
    // ---------------------------------------------------------------------------------

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::OpeningBrace)?;
        self.write_marker(UbjMarker::ClosingBrace)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.write_marker(UbjMarker::OpeningBrace)?;
        self.write_size_and_utf8_slice(variant)?;
        value.serialize(&mut (*self))?;
        self.write_marker(UbjMarker::ClosingBrace)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.write_marker(UbjMarker::OpeningBracket)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(None)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.write_marker(UbjMarker::OpeningBrace)?;
        self.write_size_and_utf8_slice(variant)?;
        self.write_marker(UbjMarker::OpeningBrace)?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_marker(UbjMarker::OpeningBrace)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_marker(UbjMarker::OpeningBrace)?;
        Ok(self)
    }

    #[cfg(not(feature = "std"))]
    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + core::fmt::Display,
    {
        todo!()
    }
}

impl<'a, W> serde::ser::SerializeSeq for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBracket)
    }
}

impl<'a, W> serde::ser::SerializeTuple for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBracket)
    }
}

impl<'a, W> serde::ser::SerializeTupleStruct for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBracket)
    }
}

impl<'a, W> serde::ser::SerializeTupleVariant for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBracket)
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.write_size_and_utf8_slice(key)?;
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBrace)?;
        self.write_marker(UbjMarker::ClosingBrace)?;
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStruct for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.write_size_and_utf8_slice(key)?;
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBrace)?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a, W> serde::ser::SerializeMap for &mut UbjSerializer<'a, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        // The given key can be of any type T that implements the serde::Serialize trait.
        // Since the type T can be any of the Serde intermediate model types, we need to delegate
        // to a separate MapKeySerializer unit struct. That implements the serde::Serializer trait
        // only for those few types that be turned into valid Universal Binary JSON keys

        let serializer = &mut MapKeySerializer { ubj: &mut (**self) };
        key.serialize(serializer)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut (**self))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_marker(UbjMarker::ClosingBrace)?;
        Ok(())
    }
}

struct MapKeySerializer<'a, 'b, W>
where
    W: IoWrite,
{
    ubj: &'b mut UbjSerializer<'a, W>,
}

impl<'a, 'b, W> serde::Serializer for &mut MapKeySerializer<'a, 'b, W>
where
    W: IoWrite,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ubj.write_size_and_utf8_slice(v)?;
        Ok(())
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("bool"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("i8"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("i16"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("i32"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("i64"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("u8"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("u16"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("u32"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("u64"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("f64"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("char"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("&[u8]"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("None"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("()"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(UbjError::IllegalKeyType("() struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!();
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        // TODO Review the following memory leak
        let msg = alloc::format!("{}::{}", name, variant);
        Err(UbjError::IllegalKeyType(Box::leak(msg.into_boxed_str())))
    }

    type SerializeSeq = serde::ser::Impossible<(), UbjError>;
    type SerializeTuple = serde::ser::Impossible<(), UbjError>;
    type SerializeTupleStruct = serde::ser::Impossible<(), UbjError>;
    type SerializeTupleVariant = serde::ser::Impossible<(), UbjError>;
    type SerializeMap = serde::ser::Impossible<(), UbjError>;
    type SerializeStruct = serde::ser::Impossible<(), UbjError>;
    type SerializeStructVariant = serde::ser::Impossible<(), UbjError>;

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(UbjError::IllegalKeyType("seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(UbjError::IllegalKeyType("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(UbjError::IllegalKeyType(name))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // TODO Review the following memory leak
        let msg = alloc::format!("{}::{}", name, variant);
        Err(UbjError::IllegalKeyType(Box::leak(msg.into_boxed_str())))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        // TODO Review the following memory leak
        let msg = alloc::format!("{}::{}", name, variant);
        Err(UbjError::IllegalKeyType(Box::leak(msg.into_boxed_str())))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(UbjError::IllegalKeyType("map"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(UbjError::IllegalKeyType(name))
    }
}

// -------------------------------------------------------------------------------------------------

/// Serializes to an IO writer
/// # Examples
/// ```rust
///
/// // Create a serializable value, such as a simple number, or a complex value for which
/// // you have implemented (or derived an implementation for) the serde::Serialize trait.
/// let value = 123_i16;
///
/// // Create a writer and serialize the value to it.
/// let mut writer = Vec::new();
/// serde_ubj::to_writer(&mut writer, &value).unwrap();
///
/// assert_eq!(writer, vec![0x55, 0x7B]);
/// ```
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<(), UbjError>
where
    W: IoWrite,
    T: serde::Serialize,
{
    let mut serializer = UbjSerializer::new(writer);
    value.serialize(&mut serializer)?;
    serializer.flush()?;
    Ok(())
}
