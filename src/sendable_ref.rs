use std::{fmt::Formatter, marker::PhantomData, ops::Deref, sync::Arc};

use serde::de::EnumAccess;
#[cfg(feature = "serde")]
use serde::{de::{Error, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

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

cfg_select!
{

    feature = "serde" =>
    {
        
        impl<T> Serialize for SendableRef<T>
            where T: Send + ?Sized + 'static + Serialize
        {

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer
            {

                match self
                {

                    SendableRef::Box(object) =>
                    {

                        serializer.serialize_newtype_variant("SendableRef", 0, "Box", object)

                    }
                    SendableRef::Static(object) =>
                    {

                        serializer.serialize_newtype_variant("SendableRef", 1, "Static", object)

                    }
                    SendableRef::Arc(object) =>
                    {

                        serializer.serialize_newtype_variant("SendableRef", 2, "Arc", object.as_ref())

                    }

                }

            }

        }

        //#[derive(Default, Debug)]
        struct SendableRefVisitor<'de, T>
             where T: Send + ?Sized + 'static + Deserialize<'de>
        {

            phantom: PhantomData<T>,
            phantom2: PhantomData<&'de T>

        }

        impl<'de, T> Default for SendableRefVisitor<'de, T>
            where T: Send + ?Sized + 'static + Deserialize<'de>
        {

            fn default() -> Self
            {

                Self
                {
                    
                    phantom: PhantomData::default(),
                    phantom2: PhantomData::default()
                
                }

            }

        }

        impl<'de, T> Visitor<'de> for SendableRefVisitor<'de, T>
            where T: Send + ?Sized + 'static + Deserialize<'de>
        {

            type Value = SendableRef<T>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
            {
                
                formatter.write_str("An enum")
                
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where A: EnumAccess<'de>
            {

                //data.variant_seed(seed)?

                //Ok(data.variant::<SendableRef<T>>()?.0)

                Ok(data.variant()?.0)

                /*
                match data.variant::<SendableRef<T>>()?
                {

                    (SendableRef::Box(object), variant) =>
                    {

                        Ok()

                    }

                }
                */

            }

        }

        impl<'de, T> Deserialize<'de> for SendableRef<T>
            where T: Send + ?Sized + 'static + Deserialize<'de>
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                let visitor = SendableRefVisitor::default();

                deserialiser.deserialize_enum("SendableRef", &["Box", "Static", "Arc"], visitor)
            
            }

        }

    }

}
