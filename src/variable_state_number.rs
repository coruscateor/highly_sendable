use std::fmt::Display;

use accessorise::impl_val_getter; //impl_get_val;

use inc_dec::IntIncDecExt;

use pastey::paste;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::VariableStateMessage;

///
/// A number used for differentiation between states.
/// 
/// It is useful when you’re filtering out irrelevant messages in pipelines that deal with networking.
/// 
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VariableStateNumber
{

    value: u32

}

impl VariableStateNumber
{

    pub fn new() -> Self
    {

        Self
        {

            value: 0

        }

    }

    impl_val_getter!(value, u32, "Gets the current number value.");

    ///
    /// Increment the value in-place and return a copy.
    /// 
    pub fn next(&mut self) -> Self
    {

       Self
       {
       
            value: self.value.wpp()

       }

    }

    ///
    /// Instantiate a new VariableStateMessage instance with the current VariableStateNumber value.
    /// 
    pub fn variable_state_message<T>(&self, message: T) -> VariableStateMessage<T>
    {

        VariableStateMessage::new(*self, message)

    }

}

impl Display for VariableStateNumber
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{}", self.value)
       
    }

}