use std::{fmt::{Display, Formatter}, ops::Deref, sync::Arc};

#[cfg(feature = "corlib")]
use corlib::text::AsStr;

use cfg_if::cfg_if;

#[cfg(feature = "serde")]
use serde::{de::{Error, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

///
/// Move immutable text between threads.
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SendableTextImmut
{

    Str(&'static str),
    ArcStr(Arc<str>)

}

impl SendableTextImmut
{

    pub fn is_str(&self) -> bool
    {

        if let SendableTextImmut::Str(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_arc_str(&self) -> bool
    {

        if let SendableTextImmut::ArcStr(_) = self
        {

            return true;

        }

        false

    }

    pub fn extract_string(self) -> Result<String, Self>
    {

        match self
        {

            SendableTextImmut::Str(val) => Err(Self::Str(val)),
            SendableTextImmut::ArcStr(val) => Err(Self::ArcStr(val))


        }

    }

    fn as_str(&self) -> &str
    {
       
        match self
        {

            SendableTextImmut::Str(val) => val,
            SendableTextImmut::ArcStr(val) => &val
            
        }


    }

}

impl Default for SendableTextImmut
{

    fn default() -> Self
    {

        Self::Str("")
       
    }

}

impl Into<String> for SendableTextImmut
{

    fn into(self) -> String
    {
        
        match self
        {

            SendableTextImmut::Str(val) => val.to_string(),
            SendableTextImmut::ArcStr(val) => val.to_string()

        }
        
    }

}

#[cfg(feature = "corlib")]
impl AsStr for SendableTextImmut
{

    fn as_str(&self) -> &str
    {
       
        self.as_str()
        
    }
    
}

impl Display for SendableTextImmut
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            SendableTextImmut::Str(val) => write!(f, "{}", val),
            SendableTextImmut::ArcStr(val) => write!(f, "{}", val)

        }
        
    }

}

impl Deref for SendableTextImmut
{

    type Target = str;

    fn deref(&self) -> &Self::Target
    {

        self.as_str()
        
    }

}

impl From<String> for SendableTextImmut
{

    fn from(value: String) -> Self
    {

        SendableTextImmut::ArcStr(Arc::from(value))

    }

}

impl From<&String> for SendableTextImmut
{

    fn from(value: &String) -> Self
    {

        SendableTextImmut::ArcStr(Arc::from(value.clone()))
        
    }

}

impl From<&'static str> for SendableTextImmut
{

    fn from(value: &'static str) -> Self
    {

        SendableTextImmut::Str(value)
        
    }

}

impl From<Arc<str>> for SendableTextImmut
{

    fn from(value: Arc<str>) -> Self
    {

        SendableTextImmut::ArcStr(value)
        
    }

}

impl From<&Arc<str>> for SendableTextImmut
{

    fn from(value: &Arc<str>) -> Self
    {

        SendableTextImmut ::ArcStr(value.clone())
        
    }

}

cfg_if!
{

    if #[cfg(feature = "serde")]
    {
    
        //#[cfg(feature = "serde")]
        impl Serialize for SendableTextImmut
        {

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer
            {

                serializer.serialize_str(self.as_str())

            }

        }

        struct SendableTextVisitor;

        impl<'de> Visitor<'de> for SendableTextVisitor
        {

            type Value = SendableTextImmut;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>
            {
                
                formatter.write_str("a String")
                
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: Error
            {

                Ok(value.to_string().into())

            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                where E: Error
            {

                Ok(value.into())

            }

        }
        
        impl<'de> Deserialize<'de> for SendableTextImmut
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                let visitor = SendableTextVisitor{};

                deserialiser.deserialize_string(visitor)
            
            }

        }
    
    }

}

