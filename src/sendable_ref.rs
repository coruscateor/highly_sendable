use std::{ops::Deref, sync::Arc};

pub enum SendableRef<T>
    where T: Send + ?Sized + 'static
    //where T: Send + Sync + 'static
{

    Box(Box<T>),
    Static(&'static T),
    Arc(Arc<T>)

}

impl<T> SendableRef<T>
    where T: Send + ?Sized + 'static //Send + Sync +
{

    pub fn is_box(&self) -> bool
    {

        matches!(self, Self::Box(_))

    }

    pub fn is_static(&self) -> bool
    {

        matches!(self, Self::Static(_))

    }

    pub fn is_arc(&self) -> bool
    {

        matches!(self, Self::Arc(_))

    }

    pub fn get_mut(&mut self) -> Option<&mut T>
    {

        match self
        {

            SendableRef::Box(val) => Some(val.as_mut()),
            SendableRef::Static(_val) => None,
            SendableRef::Arc(val) => Arc::get_mut(val)

        }

    }

}

impl<T> AsRef<T> for SendableRef<T>
    where T: Send + ?Sized + 'static
{

    fn as_ref(&self) -> &T
    {

        match self
        {

            SendableRef::Box(val) => val.as_ref(),
            SendableRef::Static(val) => val,
            SendableRef::Arc(val) => val.as_ref()

        }
        
    }

}

impl<T> Deref for SendableRef<T>
    where T: Send + ?Sized + 'static
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        
        match self
        {

            SendableRef::Box(val) => &*val,
            SendableRef::Static(val) => val,
            SendableRef::Arc(val) => &*val
            
        }

    }

}
