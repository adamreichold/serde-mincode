use std::mem::size_of;
use std::str::from_utf8;

use super::Error;

pub struct Decoder<'de> {
    buf: &'de [u8],
}

impl<'de> Decoder<'de> {
    pub fn new(buf: &'de [u8]) -> Self {
        Self { buf }
    }
}

macro_rules! impl_decode {
    ($method:ident($ty:ty)) => {
        fn $method(&mut self) -> Result<$ty, Error> {
            let (bytes, rest) = self
                .buf
                .split_at_checked(size_of::<$ty>())
                .ok_or(Error::MissingData)?;
            self.buf = rest;

            let value = <$ty>::from_ne_bytes(bytes.try_into().unwrap());

            Ok(value)
        }
    };
}

impl Decoder<'_> {
    impl_decode!(decode_i8(i8));
    impl_decode!(decode_i16(i16));
    impl_decode!(decode_i32(i32));
    impl_decode!(decode_i64(i64));
    impl_decode!(decode_i128(i128));

    impl_decode!(decode_u8(u8));
    impl_decode!(decode_u16(u16));
    impl_decode!(decode_u32(u32));
    impl_decode!(decode_u64(u64));
    impl_decode!(decode_u128(u128));

    impl_decode!(decode_f32(f32));
    impl_decode!(decode_f64(f64));

    fn decode_bytes(&mut self) -> Result<&[u8], Error> {
        let len = self.decode_u32()?;

        let (bytes, rest) = self
            .buf
            .split_at_checked(len as usize)
            .ok_or(Error::MissingData)?;
        self.buf = rest;

        Ok(bytes)
    }
}

macro_rules! impl_deserialize {
    ($method:ident($ty:ty): $decode:ident => $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let value = self.$decode()?;

            visitor.$visit(value)
        }
    };
}

impl<'de> serde::de::Deserializer<'de> for &mut Decoder<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    impl_deserialize!(deserialize_i8(i8): decode_i8 => visit_i8);
    impl_deserialize!(deserialize_i16(i16): decode_i16 => visit_i16);
    impl_deserialize!(deserialize_i32(i32): decode_i32 => visit_i32);
    impl_deserialize!(deserialize_i64(i64): decode_i64 => visit_i64);
    impl_deserialize!(deserialize_i128(i128): decode_i128 => visit_i128);

    impl_deserialize!(deserialize_u8(u8): decode_u8 => visit_u8);
    impl_deserialize!(deserialize_u16(u16): decode_u16 => visit_u16);
    impl_deserialize!(deserialize_u32(u32): decode_u32 => visit_u32);
    impl_deserialize!(deserialize_u64(u64): decode_u64 => visit_u64);
    impl_deserialize!(deserialize_u128(u128): decode_u128 => visit_u128);

    impl_deserialize!(deserialize_f32(f32): decode_f32 => visit_f32);
    impl_deserialize!(deserialize_f64(f64): decode_f64 => visit_f64);

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.decode_u8()? {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => Err(Error::InvalidBool),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bits = self.decode_u32()?;

        let value = char::from_u32(bits).ok_or(Error::InvalidChar)?;

        visitor.visit_char(value)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.decode_bytes()?;

        visitor.visit_bytes(bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.decode_bytes()?;

        visitor.visit_byte_buf(bytes.to_owned())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.decode_bytes()?;

        let value = from_utf8(bytes).map_err(|_err| Error::InvalidStr)?;

        visitor.visit_str(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.decode_bytes()?;

        let value = from_utf8(bytes).map_err(|_err| Error::InvalidStr)?;

        visitor.visit_string(value.to_owned())
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
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

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.decode_u8()? {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::InvalidOption),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.decode_u32()?;

        self.deserialize_tuple(len as usize, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(LimitedDecoder { this: self, len })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.decode_u32()?;

        visitor.visit_map(LimitedDecoder {
            this: self,
            len: len as usize,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
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
        visitor.visit_enum(self)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct LimitedDecoder<'a, 'de> {
    this: &'a mut Decoder<'de>,
    len: usize,
}

impl<'de> serde::de::SeqAccess<'de> for LimitedDecoder<'_, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.len.checked_sub(1) {
            Some(len) => {
                self.len = len;

                let value = seed.deserialize(&mut *self.this)?;

                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> serde::de::MapAccess<'de> for LimitedDecoder<'_, 'de> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.len.checked_sub(1) {
            Some(len) => {
                self.len = len;

                let value = seed.deserialize(&mut *self.this)?;

                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.this)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> serde::de::EnumAccess<'de> for &mut Decoder<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let variant_index = self.decode_u32()?;

        let deserializer = serde::de::IntoDeserializer::into_deserializer(variant_index);

        let value = seed.deserialize(deserializer)?;

        Ok((value, self))
    }
}

impl<'de> serde::de::VariantAccess<'de> for &mut Decoder<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
