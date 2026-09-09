use std::{borrow::Cow, fmt::Formatter, marker::PhantomData, ops::Deref};

use std::fmt::Debug;

use crate::SendableRef;

#[cfg(feature = "serde")]
use serde::{de::{Visitor, EnumAccess}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub enum CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static //'a + ToOwned + ?Sized + 'static // Send + Sync +
{

    SendableRef(SendableRef<T>),
    Cow(Cow<'a, T>)
    
}

impl<'a, T> CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static //'a + ToOwned + ?Sized + 'static //Clone + 'static
{

    pub fn is_sendable_ref(&self) -> bool
    {

        matches!(self, Self::SendableRef(_))

    }

    pub fn is_cow(&self) -> bool
    {

        matches!(self, Self::Cow(_))

    }

    pub fn is_sendable_ref_box(&self) -> bool
    {

        matches!(self, Self::SendableRef(SendableRef::Box(_)))

    }

    pub fn is_sendable_ref_static(&self) -> bool
    {

        matches!(self, Self::SendableRef(SendableRef::Static(_)))

    }

    pub fn is_sendable_ref_arc(&self) -> bool
    {

        matches!(self, Self::SendableRef(SendableRef::Arc(_)))

    }

    pub fn get_mut(&mut self) -> Option<&mut T>
    {

        match self
        {

            CowableRef::SendableRef(sendable_ref) => sendable_ref.get_mut(),
            CowableRef::Cow(cow) => Some(cow.to_mut())
            
        }

    }

    pub fn clone_if_not_box(&'a self) -> Option<CowableRef<'a, T>>
    {

        if self.is_sendable_ref_box()
        {

            None

        }
        else
        {

            Some(self.clone())
            
        }

    }

}

impl<'a, T> AsRef<T> for CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static
{

    fn as_ref(&self) -> &T
    {

        match self
        {

            CowableRef::SendableRef(sendable_ref) => sendable_ref.as_ref(),
            CowableRef::Cow(cow) => cow.as_ref()

        }

    }

}

impl<'a, T> Deref for CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        
        match self
        {

            CowableRef::SendableRef(sendable_ref) => &*sendable_ref,
            CowableRef::Cow(cow) => &*cow

        }

    }
    
}

impl<'a, T> PartialEq for CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static + PartialEq
{

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SendableRef(l0), Self::SendableRef(r0)) => l0 == r0,
            (Self::Cow(l0), Self::Cow(r0)) => l0 == r0,
            _ => false,
        }
    }

}

impl<'a, T> Eq for CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static + Eq
{
}

impl<'a, T> Debug for CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static + Debug
{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendableRef(arg0) => f.debug_tuple("SendableRef").field(arg0).finish(),
            Self::Cow(arg0) => f.debug_tuple("Cow").field(arg0).finish(),
        }
    }

}

cfg_select!
{

    feature = "serde" =>
    {
        
        impl<'a, T> Serialize for CowableRef<'a, T>
            where T: Send + ?Sized + Clone + 'static + Serialize
        {

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer
            {

                match self
                {

                    CowableRef::SendableRef(object) =>
                    {

                        serializer.serialize_newtype_variant("CowableRef", 0, "SendableRef", object)

                    }
                    CowableRef::Cow(object) =>
                    {

                        serializer.serialize_newtype_variant("CowableRef", 1, "Cow", object)

                    }

                }

            }

        }

        struct CowableRefVisitor<'de, 'a, T>
             where T: Send + ?Sized + Clone + 'static + Deserialize<'de>
        {

            phantom: PhantomData<T>,
            phantom2: PhantomData<&'a T>,
            phantom3: PhantomData<&'de T>

        }

        impl<'de, 'a, T> Default for CowableRefVisitor<'de, 'a, T>
            where T: Send + ?Sized + Clone + 'static + Deserialize<'de>
        {

            fn default() -> Self
            {

                Self
                {
                    
                    phantom: PhantomData::default(),
                    phantom2: PhantomData::default(),
                    phantom3: PhantomData::default()
                
                }

            }

        }

        impl<'de, 'a,  T> Visitor<'de> for CowableRefVisitor<'de, 'a, T>
            where T: Send + ?Sized + Clone + 'static + Deserialize<'de>
        {

            type Value = CowableRef<'a, T>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
            {
                
                formatter.write_str("An enum")
                
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where A: EnumAccess<'de>
            {

                Ok(data.variant()?.0)

            }

        }

        impl<'de, 'a, T> Deserialize<'de> for CowableRef<'a, T>
            where T: Send + ?Sized + Clone + 'static + Deserialize<'de>
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                let visitor = CowableRefVisitor::default();

                deserialiser.deserialize_enum("CowableRef", &["SendableRef", "Cow"], visitor)
            
            }

        }

    }
    _ =>
    {
    }

}