use super::Error;

pub struct Encoder<'a> {
    buf: &'a mut Vec<u8>,
}

impl<'a> Encoder<'a> {
    pub fn new(buf: &'a mut Vec<u8>) -> Self {
        Self { buf }
    }

    fn reborrow(&mut self) -> Encoder<'_> {
        Encoder {
            buf: &mut *self.buf,
        }
    }

    fn serialize_len(self, len: usize) -> Result<Self, Box<Error>> {
        serde::ser::Serializer::serialize_u32(self, len.try_into().expect("Excessive length"))
    }
}

macro_rules! impl_serialize {
    ($method:ident($ty:ty)) => {
        fn $method(self, value: $ty) -> Result<Self::Ok, Self::Error> {
            self.buf.extend(&value.to_ne_bytes());
            Ok(self)
        }
    };
}

impl serde::ser::Serializer for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    impl_serialize!(serialize_i8(i8));
    impl_serialize!(serialize_i16(i16));
    impl_serialize!(serialize_i32(i32));
    impl_serialize!(serialize_i64(i64));
    impl_serialize!(serialize_i128(i128));

    impl_serialize!(serialize_u8(u8));
    impl_serialize!(serialize_u16(u16));
    impl_serialize!(serialize_u32(u32));
    impl_serialize!(serialize_u64(u64));
    impl_serialize!(serialize_u128(u128));

    impl_serialize!(serialize_f32(f32));
    impl_serialize!(serialize_f64(f64));

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(value as u8)
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(value as u32)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        let this = self.serialize_len(value.len())?;
        this.buf.extend(value);
        Ok(this)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value.as_bytes())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.buf.push(0);
        Ok(self)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        self.buf.push(1);
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serialize_len(len.expect("Missing length"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let this = self.serialize_u32(variant_index)?;
        Ok(this)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let this = self.serialize_len(len.expect("Missing length"))?;
        Ok(this)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let this = self.serialize_u32(variant_index)?;
        Ok(this)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let this = self.serialize_u32(variant_index)?;
        value.serialize(this)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(variant_index)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl serde::ser::SerializeSeq for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeTuple for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeTupleStruct for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeTupleVariant for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeMap for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_key<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeStruct for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeStructVariant for Encoder<'_> {
    type Ok = Self;
    type Error = Box<Error>;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        value.serialize(self.reborrow())?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}
