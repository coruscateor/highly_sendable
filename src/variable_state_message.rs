use std::fmt::{Debug, Display};

use crate::VariableStateNumber;

use pastey::paste;

use accessorise::{impl_val_getter, impl_ref_getter, impl_mut_getter}; //{impl_get_val, impl_get_ref, impl_get_mut};

///
/// A message type that contains a ConnectionStateId and an inner message.
/// 
pub struct VariableStateMessage<T>
{

    variable_state_number: VariableStateNumber,
    message: T

}

impl<T> VariableStateMessage<T>
{

    pub fn new(variable_state_number: VariableStateNumber, message: T) -> Self
    {

        Self
        {
            
            variable_state_number,
            message
        
        }

    }

    pub fn same_number<N>(&self, message: N) -> VariableStateMessage::<N> 
    {

        VariableStateMessage::<N>::new(self.variable_state_number, message)

    }

    impl_val_getter!(variable_state_number, VariableStateNumber);

    //impl_get_val!(variable_state_id, ConnectionStateId);

    impl_ref_getter!(message, T);

    //impl_get_ref!(message, T);

    impl_mut_getter!(message, T);

    //impl_get_mut!(message, T);

    pub fn take_message(self) -> T
    {

        self.message

    }

    pub fn take_number_and_message(self) -> (VariableStateNumber, T)
    {

        (self.variable_state_number, self.message)

    }

    pub fn set_number_take_message(self, variable_state_number: &mut VariableStateNumber) -> T
    {

        *variable_state_number = self.variable_state_number; 

        self.message

    }

}

impl<T> Display for VariableStateMessage<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{{id: {0}, message: {1}}}", self.variable_state_number, self.message)
        
    }

}

impl<T> Debug for VariableStateMessage<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableStateMessage").field("variable_state_number", &self.variable_state_number).field("message", &self.message).finish()
    }

}
