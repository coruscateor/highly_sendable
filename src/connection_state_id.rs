use std::fmt::Display;

use accessorise::impl_get_val;

use inc_dec::IntIncDecSelf;

use pastey::paste;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::ConnectionMessage;

///
/// An id uniquely identifying the current connection state. Useful for filtering out irrelevant messages in pipelines that deal with networking.
/// 
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConnectionStateId
{

    id_number: u32

}

impl ConnectionStateId
{

    pub fn new() -> ConnectionStateId
    {

        Self::default()

    }

    impl_get_val!(id_number, u32, "Gets the current id number value.");

    pub fn next(&mut self) -> Self
    {

       Self
       {
       
         id_number: self.id_number.wpp()

       }

    }

    pub fn connection_message<T>(&self, message: T) -> ConnectionMessage<T>
    {

        ConnectionMessage::new(*self, message)

    }

}

impl Display for ConnectionStateId
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self.id_number)
       
    }

}