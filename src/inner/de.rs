use crate::inner::IoBufRead;
use crate::inner::err::UbjError;
use crate::inner::reader::UbjReader;
use serde::de::Visitor;

struct UbjDeserializer<R>
where
    R: IoBufRead,
{
    ubj_reader: UbjReader<R>,
}

impl<R> UbjDeserializer<R>
where
    R: IoBufRead,
{
    fn new(reader: R) -> Self {
        Self {
            ubj_reader: UbjReader::new(reader),
        }
    }
}

impl<'de, R> serde::Deserializer<'de> for &mut UbjDeserializer<R>
where
    R: IoBufRead,
{
    type Error = UbjError;

    // ---------------------------------------------------------------------------------
    // S C A L A R    values
    // ---------------------------------------------------------------------------------

    fn deserialize_any<V>(self, _isitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("deserialize_any() is not yet implemented.")
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_null()
            .and_then(|_| visitor.visit_unit())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_bool()
            .and_then(|v| visitor.visit_bool(v))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_int8()
            .and_then(|v| visitor.visit_i8(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_uint8()
            .and_then(|v| visitor.visit_u8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_int16()
            .and_then(|v| visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_int32()
            .and_then(|v| visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_int64()
            .and_then(|v| visitor.visit_i64(v))
    }

    fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(UbjError::Unsupported("i128"))
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(UbjError::Unsupported("u16"))
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(UbjError::Unsupported("u32"))
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(UbjError::Unsupported("u64"))
    }

    fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(UbjError::Unsupported("u128"))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_float32()
            .and_then(|v| visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_float64()
            .and_then(|v| visitor.visit_f64(v))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_char()
            .and_then(|v| visitor.visit_char(v))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_marked_string()
            .and_then(|s| visitor.visit_string(s))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("deserialize_str() is not yet implemented.")
        // let str: &'de str = self.ubj_reader.read_str()
        // visitor.visit_borrowed_str(str)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("deserialize_bytes() is not yet implemented.")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("deserialize_byte_buf() is not yet implemented.")
    }

    // ---------------------------------------------------------------------------------
    // C O M P O U N D   values
    // ---------------------------------------------------------------------------------

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.ubj_reader.read_null() {
            Ok(()) => visitor.visit_none(),
            Err(_) => visitor.visit_some(self),
        }
    }

    // variable-length sequences
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader.read_start_array()?;
        let seq_accessor = UbjAccessor { deserializer: self };
        visitor.visit_seq(seq_accessor)
        // NO need to read the end_array marker here
    }

    // fixed-length sequences
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader.read_start_array()?;
        let seq_accessor = UbjAccessor { deserializer: self };
        let tuple = visitor.visit_seq(seq_accessor)?;
        self.ubj_reader.read_end_array()?;
        Ok(tuple)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let id = self.ubj_reader.read_unmarked_string()?;
        visitor.visit_str(&id)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader
            .read_null()
            .and_then(|_| visitor.visit_unit())
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader.read_start_array()?;
        let seq_accessor = UbjAccessor { deserializer: self };
        let tuple = visitor.visit_seq(seq_accessor)?;
        self.ubj_reader.read_end_array()?;
        Ok(tuple)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader.read_start_object()?;
        let map_accessor = UbjAccessor { deserializer: self };
        let map = visitor.visit_map(map_accessor)?;
        // DO NOT self.ubj_reader.read_end_object()?;
        Ok(map)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader.read_start_object()?;
        let enum_accessor = UbjAccessor { deserializer: self };
        visitor.visit_enum(enum_accessor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.ubj_reader.read_start_object()?;
        let map_accessor = UbjAccessor { deserializer: self };
        let map = visitor.visit_map(map_accessor)?;
        // DO NOT self.ubj_reader.read_end_object()
        Ok(map)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!("deserialize_ignored_any() is not yet implemented.")
    }
}

// -----------------------------------------------------------------------------
//  A C C E S S O R s
// -----------------------------------------------------------------------------

struct UbjAccessor<'a, R>
where
    R: IoBufRead,
{
    deserializer: &'a mut UbjDeserializer<R>,
}

impl<'de, 'a, R> serde::de::SeqAccess<'de> for UbjAccessor<'a, R>
where
    R: IoBufRead,
{
    type Error = UbjError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        // Returning None signals the visitor about the end of the sequence,
        // while returning Some signals the visitor to continue reading the sequence.
        // This strategy does apply only for variable-length sequences and not for fixed-length ones.
        match self.deserializer.ubj_reader.read_end_array() {
            Ok(_) => Ok(None),
            Err(_) => seed.deserialize(&mut *self.deserializer).map(Some),
        }
    }
}

impl<'de, 'a, R> serde::de::MapAccess<'de> for UbjAccessor<'a, R>
where
    R: IoBufRead,
{
    type Error = UbjError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.deserializer.ubj_reader.read_end_object() {
            Ok(_) => {
                // Returning None signals the visitor about the end of the object
                Ok(None)
            }
            Err(_) => {
                // Force the use of deserialize_identifier()
                // Instead of calling seed.deserialize(&mut *self.deserializer)
                // we call with the IdentifierDeserializer, instead
                let mut identifier_deserializer = UbjIdentifierDeserializer {
                    deserializer: self.deserializer,
                };
                seed.deserialize(&mut identifier_deserializer).map(Some)
                // Returning some value signals the visitor to continue reading
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

impl<'de, 'a, R> serde::de::EnumAccess<'de> for UbjAccessor<'a, R>
where
    R: IoBufRead,
{
    type Error = UbjError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let identifier = seed.deserialize(&mut *self.deserializer)?;
        Ok((identifier, self))
    }
}

impl<'de, 'a, R> serde::de::VariantAccess<'de> for UbjAccessor<'a, R>
where
    R: IoBufRead,
{
    type Error = UbjError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.deserializer.ubj_reader.read_null()?;
        self.deserializer.ubj_reader.read_end_object()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let associated_data = seed.deserialize(&mut *self.deserializer)?;
        self.deserializer.ubj_reader.read_end_object()?;
        Ok(associated_data)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserializer.ubj_reader.read_start_array()?;
        let seq_accessor = UbjAccessor {
            deserializer: self.deserializer,
        };
        let tuple = visitor.visit_seq(seq_accessor)?;
        self.deserializer.ubj_reader.read_end_array()?;
        self.deserializer.ubj_reader.read_end_object()?;
        Ok(tuple)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserializer.ubj_reader.read_start_object()?;
        let map_accessor = UbjAccessor {
            deserializer: self.deserializer,
        };
        let map = visitor.visit_map(map_accessor)?;
        self.deserializer.ubj_reader.read_end_object()?;
        Ok(map)
    }
}

struct UbjIdentifierDeserializer<'a, R>
where
    R: IoBufRead,
{
    deserializer: &'a mut UbjDeserializer<R>,
}

impl<'de, 'a, R> serde::de::Deserializer<'de> for &mut UbjIdentifierDeserializer<'a, R>
where
    R: IoBufRead,
{
    type Error = UbjError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserializer.deserialize_identifier(visitor)
    }

    // Forward all other specific methods to deserialize_identifier as well
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 i128 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

// -------------------------------------------------------------------------------------------------

/// Deserialize from an IO buffering reader into a Rust value of type `T`.
///
/// Be aware that, this function requires a buffered reader
/// #Example
/// ```rust, ignore
/// use core::error;
/// use std::{fs, io};
///
/// fn main() -> Result<(), Box<dyn error::Error>> {
///     // Create a buffered reader from a file
///     let file = fs::File::open("file.ubj")?;
///     let mut reader = io::BufReader::new(file);
///
///     // Deserialize the value from the reader
///     let value: i32 = serde_ubj::from_buf_reader(&mut reader)?;
///     Ok(())
/// }
/// ```
pub fn from_buf_reader<'de, R, T>(reader: &mut R) -> Result<T, UbjError>
where
    R: IoBufRead,
    T: serde::Deserialize<'de>,
{
    let mut deserializer = UbjDeserializer::new(reader);
    serde::Deserialize::deserialize(&mut deserializer)
}

/// Deserialize from a vector (in-memory buffer) of bytes into a Rust value of type `T`.
pub fn from_vec<'de, T>(vec: Vec<u8>) -> Result<T, UbjError>
where
    T: serde::Deserialize<'de>,
{
    from_buf_reader(&mut vec.as_slice())
}
