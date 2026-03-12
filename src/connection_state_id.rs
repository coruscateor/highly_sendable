use std::fmt::Display;

use accessorise::impl_get_val;

use inc_dec::IntIncDecSelf;

use pastey::paste;

use crate::ConnectionMessage;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

    /*
    pub fn next_some(&mut self) -> Option<ConnectionStateId>
    {

        Some(self.next())

    }
    */

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