use crate::prelude::*;
use crate::inner::err::UbjError;
use crate::inner::markers::UbjMarker;

struct UbjSerializer<'a, W>
where
    W: IoWriter
{
    #[deprecated(since = "0.3.0", note = "Use `UbjWriter` instead")]
    writer: &'a mut W,
    // TODO writer: UbjWriter<W>,
}

impl<'a, W> UbjSerializer<'a, W>
where
    W: IoWriter,
{

    pub fn write_unmarked_string(&mut self, v: &str) -> UbjResult<()> {
        self.write_marker(UbjMarker::Str)?;
        self.write_unmarked_string(v)
    }

    fn write_none(&mut self) -> UbjResult<()> {
        self.write_marker(UbjMarker::Null)
    }

    // TODO fn write_huge(&mut self, number: u64) -> UbjResult<()> {


}

impl<'a, W> serde::Serializer for &mut UbjSerializer<'a, W>
where
    W: IoWriter,
{
    type Ok = ();
    type Error = UbjError;




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
        self.write_unmarked_string(variant)?;
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
        self.write_unmarked_string(variant)?;
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
    W: IoWriter,
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
    W: IoWriter,
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
    W: IoWriter,
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
    W: IoWriter,
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
    W: IoWriter,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.write_unmarked_string(key)?;
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
    W: IoWriter,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.write_unmarked_string(key)?;
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
    W: IoWriter,
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
    W: IoWriter,
{
    ubj: &'b mut UbjSerializer<'a, W>,
}

impl<'a, 'b, W> serde::Serializer for &mut MapKeySerializer<'a, 'b, W>
where
    W: IoWriter,
{
    type Ok = ();
    type Error = UbjError;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ubj.write_unmarked_string(v)?;
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

/// Serializes a Rust value of type `T` to an IO writer
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
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> UbjResult<()>
where
    W: IoWrite,
    T: serde::Serialize,
{
    let mut serializer = UbjSerializer::new(writer);
    value.serialize(&mut serializer)?;
    serializer.flush()?;
    Ok(())
}
