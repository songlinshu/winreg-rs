// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::fmt;
use std::mem;
use super::{Encoder, EncoderError, EncodeResult, ENCODER_SAM};
use super::EncoderState::*;
// use std::convert::Into;
use serde::ser::*;

// pub struct RegSerializer {
//     key: RegKey,
// }

// impl RegSerializer {
//     pub fn new(key: &RegKey) -> Self {
//         RegSerializer { key: key.open_subkey("").unwrap() }
//     }
// }

impl Error for EncoderError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        EncoderError::SerializerError(format!("{}", msg))
    }
}


impl<'a> Serializer for &'a mut Encoder {
    type Ok = ();
    type Error = EncoderError;

    type SerializeSeq = RegSerializeSeqStub;
    type SerializeTuple = RegSerializeTupleStub;
    type SerializeTupleStruct = RegSerializeTupleStructStub;
    type SerializeTupleVariant = RegSerializeTupleVariantStub;
    type SerializeMap = RegCompound<'a>;
    type SerializeStruct = RegSerializeStructStub<'a>;
    type SerializeStructVariant = RegSerializeStructVariantStub;

    fn serialize_bool(self, value: bool) -> EncodeResult<Self::Ok> {
        self.serialize_u32(value as u32)
    }

    fn serialize_i8(self, value: i8) -> EncodeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i16(self, value: i16) -> EncodeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i32(self, value: i32) -> EncodeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> EncodeResult<Self::Ok> {
        let s = value.to_string();
        emit_value!(self, s)
    }

    fn serialize_u8(self, value: u8) -> EncodeResult<Self::Ok> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u16(self, value: u16) -> EncodeResult<Self::Ok> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u32(self, value: u32) -> EncodeResult<Self::Ok> {
        emit_value!(self, value)
    }

    fn serialize_u64(self, value: u64) -> EncodeResult<Self::Ok> {
        emit_value!(self, value)
    }

    fn serialize_f32(self, value: f32) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_f32")
    }

    fn serialize_f64(self, value: f64) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_f64")
    }

    fn serialize_char(self, value: char) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_char")
    }

    fn serialize_str(self, value: &str) -> EncodeResult<Self::Ok> {
        emit_value!(self, value)
    }

    fn serialize_bytes(self, value: &[u8]) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_bytes")
    }

    fn serialize_none(self) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_none")
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_some")
    }

    fn serialize_unit(self) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_unit")
    }

    fn serialize_unit_struct(self, name: &'static str) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_unit_struct")
    }

    fn serialize_unit_variant(self,
                              name: &'static str,
                              variant_index: u32,
                              variant: &'static str)
                              -> EncodeResult<Self::Ok> {
        no_impl!("serialize_unit_variant")
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self,
                                                       name: &'static str,
                                                       value: &T)
                                                       -> EncodeResult<Self::Ok> {
        no_impl!("serialize_newtype_struct")
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self,
                                                        name: &'static str,
                                                        variant_index: u32,
                                                        variant: &'static str,
                                                        value: &T)
                                                        -> EncodeResult<Self::Ok> {
        no_impl!("serialize_newtype_variant")
    }

    fn serialize_seq(self, len: Option<usize>) -> EncodeResult<Self::SerializeSeq> {
        no_impl!("serialize_seq")
    }

    fn serialize_tuple(self, len: usize) -> EncodeResult<Self::SerializeTuple> {
        no_impl!("serialize_tuple")
    }

    fn serialize_tuple_struct(self,
                              name: &'static str,
                              len: usize)
                              -> EncodeResult<Self::SerializeTupleStruct> {
        no_impl!("serialize_tuple_struct")
    }

    fn serialize_tuple_variant(self,
                               name: &'static str,
                               variant_index: u32,
                               variant: &'static str,
                               len: usize)
                               -> EncodeResult<Self::SerializeTupleVariant> {
        no_impl!("serialize_tuple_variant")
    }

    fn serialize_map(self, len: Option<usize>) -> EncodeResult<Self::SerializeMap> {
        Ok(RegCompound { enc: self })
    }

    fn serialize_struct(self,
                        name: &'static str,
                        len: usize)
                        -> EncodeResult<Self::SerializeStruct> {
        // )
        match mem::replace(&mut self.state, Start) {
            Start => {
                // root structure
                Ok(RegSerializeStructStub { enc: self, is_root: true })
            }
            NextKey(ref s) => {
                // nested structure
                match self.keys[self.keys.len() - 1]
                    .create_subkey_transacted_with_flags(&s, &self.tr, ENCODER_SAM) {
                    Ok(subkey) => {
                        self.keys.push(subkey);
                        Ok(RegSerializeStructStub { enc: self, is_root: true })
                    }
                    Err(err) => Err(EncoderError::IoError(err)),
                }
            }
        }
    }

    fn serialize_struct_variant(self,
                                name: &'static str,
                                variant_index: u32,
                                variant: &'static str,
                                len: usize)
                                -> EncodeResult<Self::SerializeStructVariant> {
        no_impl!("serialize_struct_variant")
    }
}

pub struct RegSerializeSeqStub {}

impl SerializeSeq for RegSerializeSeqStub {
    type Ok = ();
    type Error = EncoderError;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeSeq::serialize_element")
    }
    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeSeq::end")
    }
}

pub struct RegSerializeTupleStub {}

impl SerializeTuple for RegSerializeTupleStub {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTuple::serialize_element")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTuple::end")
    }
}

pub struct RegSerializeTupleStructStub {}

impl SerializeTupleStruct for RegSerializeTupleStructStub {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleStruct::serialize_field")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleStruct::end")
    }
}

pub struct RegSerializeTupleVariantStub {}

impl SerializeTupleVariant for RegSerializeTupleVariantStub {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleVariant::serialize_field")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleVariant::end")
    }
}

pub struct RegCompound<'a> {
    enc: &'a mut Encoder,
}

impl<'a> SerializeMap for RegCompound<'a> {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeMap::serialize_key")
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeMap::serialize_value")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeMap::end")
    }
}

pub struct RegSerializeStructStub<'a> {
    enc: &'a mut Encoder,
    is_root: bool
}

impl<'a> SerializeStruct for RegSerializeStructStub<'a> {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              key: &'static str,
                                              value: &T)
                                              -> EncodeResult<Self::Ok> {
        self.enc.state = NextKey(String::from(key));
        value.serialize(&mut *self.enc)
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        if self.is_root {
            self.enc.keys.pop();
        }
        Ok(())
    }
}

pub struct RegSerializeStructVariantStub {}

impl SerializeStructVariant for RegSerializeStructVariantStub {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              key: &'static str,
                                              value: &T)
                                              -> EncodeResult<Self::Ok> {
        no_impl!("SerializeStructVariant::serialize_field")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeStructVariant::end")
    }
}

