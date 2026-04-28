
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use accessorise::{impl_get_ref_mut, impl_get_ref, impl_get_mut};

use pastey::paste;

use std::fmt::Debug;

use crate::BasicStatus;

pub type WorkInProgressMessageOption<T, S = BasicStatus, M = ()> = WorkInProgressMessage<Option<T>, S, M>;

pub type WorkInProgressMessageResult<T, E = (), S = BasicStatus, M = ()> = WorkInProgressMessage<Result<T, E>, S, M>; 

///
/// For reporting the progress of work being done and sending any values between threads, tasks or processes. 
/// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkInProgressMessage<T, S = BasicStatus, M = ()>
{

    value: T,
    status: S,
    meta_data: M

}

impl<T, S, M> WorkInProgressMessage<T, S, M>
{

    pub fn new(value: T, status: S, meta_data: M) -> Self
    {

        Self
        {
            
            value,
            status,
            meta_data
        
        }

    }

    impl_get_ref_mut!(value, T);

    impl_get_ref_mut!(status, S);

    impl_get_ref_mut!(meta_data, M);

    pub fn take(self) -> (T, S, M)
    {

        (self.value, self.status, self.meta_data)

    }

    pub fn take_value(self) -> T
    {

        self.value

    }

    pub fn take_status(self) -> S
    {

        self.status

    }

    pub fn take_meta_data(self) -> M
    {

        self.meta_data

    }

}

impl<T, S, M> WorkInProgressMessage<T, S, M>
    where M: Default
{

    pub fn value_status(value: T, status: S) -> Self
    {

        Self
        {
            
            value,
            status,
            meta_data: Default::default()
        
        }

    }

}

impl<T, S, M> Clone for WorkInProgressMessage<T, S, M>
    where T: Clone,
          S: Clone,
          M: Clone
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            value: self.value.clone(),
            status: self.status.clone(),
            meta_data: self.meta_data.clone()
        }

    }

}

impl<T, S, M> Default for WorkInProgressMessage<T, S, M>
    where T: Default,
          S: Default,
          M: Default
{

    fn default() -> Self
    {
        Self
        {

            value: Default::default(),
            status: Default::default(),
            meta_data: Default::default()

        }

    }

}

impl<T, S, M> Debug for  WorkInProgressMessage<T, S, M>
    where T: Debug,
          S: Debug,
          M: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        f.debug_struct("WorkInProgressMessage").field("value", &self.value).field("status", &self.status).field("meta_data", &self.meta_data).finish()

    }

}
