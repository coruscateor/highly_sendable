use std::{ops::{Deref, DerefMut}, sync::Arc};


pub enum SendableRef<T>
    where T: 'static
    //where T: Send + Sync + 'static
{

    Box(Box<T>),
    Static(&'static T),
    Arc(Arc<T>)

}

impl<T> SendableRef<T>
    where T: 'static //Send + Sync +
{

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
