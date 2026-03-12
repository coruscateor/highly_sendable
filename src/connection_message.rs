use std::fmt::{Debug, Display};

use crate::ConnectionStateId;

use pastey::paste;

use accessorise::{impl_get_val, impl_get_ref, impl_get_mut};

///
/// A message type that contains a ConnectionStateId and an inner message.
/// 
pub struct ConnectionMessage<T>
{

    connection_state_id: ConnectionStateId,
    message: T

}

impl<T> ConnectionMessage<T>
{

    pub fn new(connection_state_id: ConnectionStateId, message: T) -> Self
    {

        Self
        {
            
            connection_state_id,
            message
        
        }

    }

    pub fn same_id<N>(&self, message: N) -> ConnectionMessage::<N> 
    {

        ConnectionMessage::<N>::new(self.connection_state_id, message)

    }

    impl_get_val!(connection_state_id, ConnectionStateId);

    impl_get_ref!(message, T);

    impl_get_mut!(message, T);

    pub fn take_message(self) -> T
    {

        self.message

    }

    pub fn take_id_and_message(self) -> (ConnectionStateId, T)
    {

        (self.connection_state_id, self.message)

    }

    pub fn set_id_take_message(self, connection_state_id: &mut ConnectionStateId) -> T
    {

        *connection_state_id = self.connection_state_id; 

        self.message

    }

}

impl<T> Display for ConnectionMessage<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "{{id: {0}, message: {1}}}", self.connection_state_id, self.message)
        
    }

}

impl<T> Debug for ConnectionMessage<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionMessage").field("connection_state_id", &self.connection_state_id).field("message", &self.message).finish()
    }

}