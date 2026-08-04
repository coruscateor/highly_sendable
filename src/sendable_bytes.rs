use std::{ops::Deref, sync::Arc, fmt::Formatter};

#[cfg(feature = "bytes")]
use bytes::Bytes;

use crate::text::SendableText;

#[cfg(feature = "serde")]
use serde::{de::{Error, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

pub enum SendableBytes
{

    Vec(Vec<u8>),
    Slice(&'static [u8]),
    ArcSlice(Arc<[u8]>),
    #[cfg(feature = "bytes")]
    Bytes(Bytes),
    SendableText(SendableText)

}

impl SendableBytes
{

    pub fn string(val: String) -> SendableBytes
    {

        Self::SendableText(SendableText::String(val))

    }

    pub fn str(val: &'static str) -> SendableBytes
    {

        Self::SendableText(SendableText::Str(val))

    }

    pub fn arc_str(val: Arc<str>) -> SendableBytes
    {

        Self::SendableText(SendableText::ArcStr(val))

    }

    pub fn is_vec(&self) -> bool
    {

        matches!(self, Self::Vec(_))

    }

    pub fn is_slice(&self) -> bool
    {

        matches!(self, Self::Slice(_))

    }

    pub fn is_arc_slice(&self) -> bool
    {

        matches!(self, Self::ArcSlice(_))

    }

    #[cfg(feature = "bytes")]
    pub fn is_bytes(&self) -> bool
    {

        matches!(self, Self::Bytes(_))

    }

    pub fn is_sendable_text(&self) -> bool
    {

        matches!(self, Self::SendableText(_))

    }

    pub fn as_slice(&self) -> &[u8]
    {

        match self
        {

            SendableBytes::Vec(items) =>
            {

                items.as_slice()

            }
            SendableBytes::Slice(items) =>
            {

                *items

            }
            SendableBytes::ArcSlice(items) =>
            {

                items

            }
            #[cfg(feature = "bytes")]
            SendableBytes::Bytes(bytes) =>
            {

                bytes.iter().as_slice()

            }
            SendableBytes::SendableText(item) =>
            {

                item.as_bytes()

            }

        }

    }

    pub fn len(&self) -> usize
    {

        match self
        {

            SendableBytes::Vec(items) => items.len(),
            SendableBytes::Slice(items) => items.len(),
            SendableBytes::ArcSlice(items) => items.len(),
            SendableBytes::Bytes(bytes) => bytes.len(),
            SendableBytes::SendableText(sendable_text) => sendable_text.len()

        }

    }

    pub fn capacity(&self) -> usize
    {

        match self
        {

            SendableBytes::Vec(items) => items.capacity(),
            SendableBytes::Slice(items) => items.len(),
            SendableBytes::ArcSlice(items) => items.len(),
            SendableBytes::Bytes(bytes) => bytes.len(),
            SendableBytes::SendableText(sendable_text) => sendable_text.capacity()

        }

    }

    pub fn len_is_at_capacity(&self) -> bool
    {

        match self
        {

            SendableBytes::Vec(items) => items.len() == items.capacity(),
            SendableBytes::Slice(_items) => true,
            SendableBytes::ArcSlice(_items) => true,
            SendableBytes::Bytes(_bytes) => true,
            SendableBytes::SendableText(sendable_text) => sendable_text.len_is_at_capacity()

        }

    }

}

impl Default for SendableBytes
{

    fn default() -> Self
    {
        
        Self::Slice(&[])

    }

}

impl Into<Vec<u8>> for SendableBytes
{

    fn into(self) -> Vec<u8>
    {
        
        match self
        {

            SendableBytes::Vec(items) =>
            {

                items

            }
            SendableBytes::Slice(items) =>
            {

                items.into()

            }
            SendableBytes::ArcSlice(items) =>
            {

                (*items).into()

               //let vec = Vec::with_capacity(items.len());

               //Vec::fr

               //vec

            }
            #[cfg(feature = "bytes")]
            SendableBytes::Bytes(bytes) =>
            {

                bytes.into()

            }
            SendableBytes::SendableText(sendable_text) =>
            {


                sendable_text.as_bytes().into()

            }

        }

    }

}

impl Deref for SendableBytes
{

    type Target = [u8];

    fn deref(&self) -> &Self::Target
    {
        
        self.as_slice()

    }

}

impl AsRef<[u8]> for SendableBytes
{

    fn as_ref(&self) -> &[u8]
    {

        self.as_slice()

    }
    
}

impl From<Vec<u8>> for SendableBytes
{

    fn from(value: Vec<u8>) -> Self
    {
        
        Self::Vec(value)
        
    }

}

impl From<&Vec<u8>> for SendableBytes
{

    fn from(value: &Vec<u8>) -> Self
    {
        
        Self::Vec(value.clone())

    }

}

impl From<&'static [u8]> for SendableBytes
{

    fn from(value: &'static [u8]) -> Self
    {

        Self::Slice(value)
        
    }

}

impl From<Arc<[u8]>> for SendableBytes
{

    fn from(value: Arc<[u8]>) -> Self
    {
        
        Self::ArcSlice(value)

    }

}

impl From<&Arc<[u8]>> for SendableBytes
{

    fn from(value: &Arc<[u8]>) -> Self
    {
        
        Self::ArcSlice(value.clone())
        
    }

}

#[cfg(feature = "bytes")]
impl From<Bytes> for SendableBytes
{

    fn from(value: Bytes) -> Self
    {
        
        Self::Bytes(value)

    }

}

#[cfg(feature = "bytes")]
impl From<&Bytes> for SendableBytes
{

    fn from(value: &Bytes) -> Self
    {
        
        Self::Bytes(value.clone())

    }

}

impl From<SendableText> for SendableBytes
{

    fn from(value: SendableText) -> Self
    {
        
        Self::SendableText(value)
        
    }

}

impl From<&SendableText> for SendableBytes
{

    fn from(value: &SendableText) -> Self
    {
        
        Self::SendableText(value.clone())
        
    }

}

cfg_select!
{

    feature = "serde" =>
    {
        
        impl Serialize for SendableBytes
        {

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer
            {

                serializer.serialize_bytes(self)

            }

        }

        struct SendableBytesVisitor;

        impl<'de> Visitor<'de> for SendableBytesVisitor
        {

            type Value = SendableBytes;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
            {
                
                formatter.write_str("A byte array.")
                
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error
            {

                Ok(SendableBytes::Vec(v.into()))

            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: Error
            {

                Ok(SendableBytes::Vec(v.into()))

            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {

                Ok(SendableBytes::Vec(v))

            }

        }
        
        impl<'de> Deserialize<'de> for SendableBytes
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                let visitor = SendableBytesVisitor{};

                deserialiser.deserialize_byte_buf(visitor)
            
            }

        }
    
    }
    _ =>
    {
    }

}

