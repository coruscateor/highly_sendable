use std::{borrow::Cow, ops::Deref, sync::Arc};

use crate::SendableRef;


pub enum CowableRef<'a, T>
    where T: Clone + 'static // Send + Sync +
{

    Ref(SendableRef<T>),
    Cow(Cow<'a, T>)
    
}

impl<'a, T> CowableRef<'a, T>
    where T: Clone + 'static
{

    pub fn is_ref(&self) -> bool
    {

        matches!(self, Self::Ref(_))

    }

    pub fn is_cow(&self) -> bool
    {

        matches!(self, Self::Cow(_))

    }

    pub fn is_ref_box(&self) -> bool
    {

        matches!(self, Self::Ref(SendableRef::Box(_)))

    }

    pub fn is_ref_static(&self) -> bool
    {

        matches!(self, Self::Ref(SendableRef::Static(_)))

    }

    pub fn is_ref_arc(&self) -> bool
    {

        matches!(self, Self::Ref(SendableRef::Arc(_)))

    }

    pub fn get_mut(&mut self) -> Option<&mut T>
    {

        match self
        {

            CowableRef::Ref(sendable_ref) => sendable_ref.get_mut(),
            CowableRef::Cow(cow) => Some(cow.to_mut())
            
        }

    }

}

impl<'a, T> AsRef<T> for CowableRef<'a, T>
    where T: Clone + 'static
{

    fn as_ref(&self) -> &T
    {

        match self
        {

            CowableRef::Ref(sendable_ref) => sendable_ref.as_ref(),
            CowableRef::Cow(cow) => cow.as_ref()

        }

    }

}

impl<'a, T> Deref for CowableRef<'a, T>
    where T: Clone + 'static
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        
        match self
        {

            CowableRef::Ref(sendable_ref) => &*sendable_ref,
            CowableRef::Cow(cow) => &*cow

        }

    }
    
}
