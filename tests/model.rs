// By default, standard Rust types like Vec<u8> or &[u8] do not trigger serialize_bytes().
// Instead, they are treated as generic sequences and invoke serialize_seq(), which processes
// each byte individually as an u8 type (see https://docs.rs/serde/1.0.228/serde/trait.Serializer.html#tymethod.serialize_bytes)
//
// serialize_bytes() is invoked only when a type explicitly calls it within its Serialize
// implementation. This happens when using the serde_bytes crate, and when the data is wrapped
// in serde_bytes::Bytes (for &[u8]) or serde_bytes::ByteBuf (for Vec<u8>)
//

use serde::{Deserialize, Serialize};


// #[derive(Serialize)]
// pub struct MyBytesWrapper<'a> {
//     #[serde(with = "serde_bytes")]
//     pub bytes: &'a [u8],
//
//     //#[serde(with = "serde_bytes")]
//     //byte_buf: Vec<u8>,
//
//     //#[serde(with = "serde_bytes")]
//     //byte_array: [u8; 3],
// }

// -------------------------------------------------------------------------------------------------
// A struct with no fields
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MyUnitStruct;

// impl<'de> Deserialize<'de> for MyUnitStruct {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         struct MyUnitStructVisitor;
//
//         impl<'de> de::Visitor<'de> for MyUnitStructVisitor {
//             type Value = MyUnitStruct;
//             fn expecting(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
//                 f.write_str("my unit struct")
//             }
//             fn visit_unit<E>(self) -> Result<Self::Value, E> {
//                 Ok(MyUnitStruct)
//             }
//         }
//
//         deserializer.deserialize_unit_struct(
//             "MyUnitStruct",
//             MyUnitStructVisitor {}
//         )
//     }
// }


// -------------------------------------------------------------------------------------------------
// A struct with a single unnamed field
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
// #[serde(transparent)]
pub struct MyNewtypeStruct(pub i8);

// impl Serialize for MyNewtypeStruct {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer
//     {
//         serializer.serialize_newtype_struct("MyNewtypeStruct", &self.0)
//     }
// }
//
// impl<'de> Deserialize<'de> for MyNewtypeStruct {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct MyNewtypeStructVisitor;
//
//         impl<'de> de::Visitor<'de> for MyNewtypeStructVisitor {
//             type Value = MyNewtypeStruct;
//             fn expecting(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
//                 f.write_str("my newtype struct")
//             }
//
//             fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 let v = i8::deserialize(deserializer)?;
//                 Ok(MyNewtypeStruct(v))
//             }
//         }
//
//         deserializer.deserialize_newtype_struct("MyNewtypeStruct", MyNewtypeStructVisitor)
//     }
// }


// -------------------------------------------------------------------------------------------------
// A struct with multiple unnamed fields
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MyTupleStruct(pub i8, pub i16, pub i32);




// -------------------------------------------------------------------------------------------------
// A struct with named fields
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MyFieldsStruct {
   pub x: i8,
   pub y: bool,
   pub z: String
}

// use ser::SerializeStruct;
// impl Serialize for MyStruct {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("Struct", 2)?;
//         state.serialize_field("x", &self.x)?;
//         state.serialize_field("y", &self.y)?;
//         state.end()
//     }
// }

// impl<'de> Deserialize<'de> for MyStruct {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         struct MyStructVisitor;
//
//         impl<'de> de::Visitor<'de> for MyStructVisitor {
//             type Value = MyStruct;
//
//             fn expecting(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
//                 f.write_unmarked_string("a struct")
//             }
//
//             fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: de::MapAccess<'de>
//             {
//             }
//         }
//
//         deserializer.deserialize_struct("Struct", &["x", "y"], MyStructVisitor)
//     }
// }
//
//
// // -------------------------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
// #[repr(u8)]
// #[serde(tag = "type")]
// #[serde(tag = "t", content = "c")]
// #[serde(untagged)]
pub enum MyEnum {
    // a variant with no associated data
    MyUnitVariant,

    // This variant borrows directly from the input buffer
    // TODO #[serde(borrow)]
    // TODO MyNewtypeVariant(&'a str),

    // a variant with a single i32 field
    MyNewtypeVariant(String),

    // a variant with multiple unnamed associated fields
    MyTupleVariant(i8, i16, i32),

    // a variant with named associated fields
    MyStructVariant { x: i8, y: bool, z: String },
}

//
// impl Serialize for MyEnum {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer
//     {
//         match self {
//             MyEnum::MyUnitVariant => {
//                 serializer.serialize_unit_variant("MyEnum", 0, "MyUnitVariant")
//             }
//             MyEnum::MyNewtypeVariant(v) => {
//                 serializer.serialize_newtype_variant("MyEnum", 1, "MyNewtypeVariant", v)
//             },
//             MyEnum::MyTupleVariant(v1, v2, v3) => {
//                 let mut s = serializer.serialize_tuple_variant("MyEnum", 2, "MyTupleVariant", 3)?;
//                 s.serialize_field(v1)?;
//                 s.serialize_field(v2)?;
//                 s.serialize_field(v3)?;
//                 s.end()
//             },
//             MyEnum::MyStructVariant { x, y }  => {
//                 let mut s = serializer.serialize_struct_variant("MyEnum", 3, "MyStructVariant", 2)?;
//                 s.serialize_field("x", x)?;
//                 s.serialize_field("y", y)?;
//                 s.end()
//             },
//         }
//     }
// }
//
//
// enum MyEnumIdentifier {
//     MyUnitVariant,
//     MyNewtypeVariant,
//     MyTupleVariant,
//     MyStructVariant
// }
//
// impl<'de> Deserialize<'de> for MyEnumIdentifier {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct MyEnumIdentifierVisitor;
//
//         impl<'de> de::Visitor<'de> for MyEnumIdentifierVisitor {
//             type Value = MyEnumIdentifier;
//             fn expecting(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
//                 f.write_str("my enum identifier")
//             }
//
//             fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
//             where
//                 E: de::Error
//             {
//                 match v {
//                     0 => Ok(MyEnumIdentifier::MyUnitVariant),
//                     1 => Ok(MyEnumIdentifier::MyNewtypeVariant),
//                     2 => Ok(MyEnumIdentifier::MyTupleVariant),
//                     3 => Ok(MyEnumIdentifier::MyStructVariant),
//                     _ => Err(de::Error::custom(format!("invalid enum identifier index: {}", v)))
//                 }
//             }
//
//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                 E: de::Error
//             {
//                 match v {
//                     "MyUnitVariant"    => Ok(MyEnumIdentifier::MyUnitVariant),
//                     "MyNewtypeVariant" => Ok(MyEnumIdentifier::MyNewtypeVariant),
//                     "MyTupleVariant"   => Ok(MyEnumIdentifier::MyTupleVariant),
//                     "MyStructVariant"  => Ok(MyEnumIdentifier::MyStructVariant),
//                     _                   => Err(de::Error::custom(format!("invalid enum identifier name: {}", v)))
//                 }
//             }
//         }
//
//         let visitor = MyEnumIdentifierVisitor {};
//         deserializer.deserialize_identifier(visitor)
//     }
// }
//
// impl<'de> Deserialize<'de> for MyEnum {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct MyEnumVisitor;
//
//         impl<'de> de::Visitor<'de> for MyEnumVisitor {
//             type Value = MyEnum;
//
//             fn expecting(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
//                 f.write_str("my enum")
//             }
//
//             fn visit_enum<A>(self, enum_access: A) -> Result<Self::Value, A::Error>
//             where
//                 A: de::EnumAccess<'de>,
//             {
//
//                 // struct MyTupleVariantVisitor;
//                 // impl<'de> de::Visitor<'de> for MyTupleVariantVisitor {
//                 //     type Value = MyEnum;
//                 //
//                 //     fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
//                 //         f.write_str("my tuple variant")
//                 //     }
//                 //
//                 //     fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//                 //     where
//                 //         E: Error
//                 //     {
//                 //         Ok(MyEnum::MyNewtypeVariant(v))
//                 //     }
//                 // }
//
//
//                 let (field, variant_access): (MyEnumIdentifier, _) = enum_access.variant()?;
//                 match field {
//                     MyEnumIdentifier::MyUnitVariant    => {
//                         variant_access.unit_variant()?;
//                         Ok(MyEnum::MyUnitVariant)
//                     },
//                     MyEnumIdentifier::MyNewtypeVariant => {
//                         let v: String = de::VariantAccess::newtype_variant(variant_access)?;
//                         Ok(MyEnum::MyNewtypeVariant(v))
//                     },
//                     MyEnumIdentifier::MyTupleVariant   => {
//                         ???
//                     },
//                     MyEnumIdentifier::MyStructVariant  => {
//                         ???
//                     }
//                 }
//             }
//         }
//
//         deserializer.deserialize_enum(
//             "MyEnum",
//             &["MyUnitVariant", "MyNewtypeVariant", "MyTupleVariant", "MyStructVariant"],
//             MyEnumVisitor {}
//         )
//     }
// }