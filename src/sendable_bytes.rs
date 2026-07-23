use std::{ops::Deref, sync::Arc};

#[cfg(feature = "bytes")]
use bytes::Bytes;

use crate::text::SendableText;

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
