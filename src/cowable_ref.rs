use std::{borrow::Cow, fmt::Formatter, marker::PhantomData, ops::Deref};

use crate::SendableRef;

#[cfg(feature = "serde")]
use serde::{de::{Error, Visitor, EnumAccess}, Deserialize, Deserializer, Serialize, Serializer};

pub enum CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static //'a + ToOwned + ?Sized + 'static // Send + Sync +
{

    Ref(SendableRef<T>),
    Cow(Cow<'a, T>)
    
}

impl<'a, T> CowableRef<'a, T>
    where T: Send + ?Sized + Clone + 'static //'a + ToOwned + ?Sized + 'static //Clone + 'static
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
    where T: Send + ?Sized + Clone + 'static
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
    where T: Send + ?Sized + Clone + 'static
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

                    CowableRef::Ref(object) =>
                    {

                        serializer.serialize_newtype_variant("CowableRef", 0, "Ref", object)

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

                deserialiser.deserialize_enum("CowableRef", &["Ref", "Cow"], visitor)
            
            }

        }

    }
    _ =>
    {
    }

}