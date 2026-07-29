use std::fmt::Display;

use accessorise::impl_val_getter; //impl_get_val;

use inc_dec::IntIncDecSelf;

use pastey::paste;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::VariableStateMessage;

///
/// An id uniquely identifying the current connection state. Useful for filtering out irrelevant messages in pipelines that deal with networking.
/// 
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VariableStateNumber
{

    variable_state_number: u32

}

impl VariableStateNumber
{

    pub fn new() -> Self
    {

        Self::default()

    }

    impl_val_getter!(variable_state_number, u32, "Gets the current id number value.");

    //impl_get_val!(id_number, u32, "Gets the current id number value.");

    pub fn next(&mut self) -> Self
    {

       Self
       {
       
            variable_state_number: self.variable_state_number.wpp()

       }

    }

    pub fn variable_state_message<T>(&self, message: T) -> VariableStateMessage<T>
    {

        VariableStateMessage::new(*self, message)

    }

}

impl Display for VariableStateNumber
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self.variable_state_number)
       
    }

}