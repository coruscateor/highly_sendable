
use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub enum EssentialStatus
{

    #[default]
    InProgress,
    NotInProgress

}

impl EssentialStatus
{

    pub fn is_in_progress(&self) -> bool
    {

        matches!(self, Self::InProgress)

    }

    pub fn is_not_in_progress(&self) -> bool
    {

        matches!(self, Self::NotInProgress)

    }

}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub enum PauseableStatus
{

    #[default]
    InProgress,
    Paused,
    Ended

}

impl PauseableStatus
{

    pub fn is_in_progress(&self) -> bool
    {

        matches!(self, Self::InProgress)

    }

    pub fn is_paused(&self) -> bool
    {

        matches!(self, Self::Paused)

    }

    pub fn is_ended(&self) -> bool
    {

        matches!(self, Self::Ended)

    }

}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub enum BasicStatus
{

    #[default]
    InProgress,
    Ended

}

impl BasicStatus
{

    pub fn is_in_progress(&self) -> bool
    {

        matches!(self, Self::InProgress)

    }

    pub fn is_ended(&self) -> bool
    {

        matches!(self, Self::Ended)

    }

}

pub type BasicStatusU32 = BasicStatusWithItem<u32>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BasicStatusWithItem<T>
{

    InProgress(T),
    Ended

}

impl<T> BasicStatusWithItem<T>
{

    pub fn is_in_progress(&self) -> bool
    {

        matches!(self, Self::InProgress(_))

    }

    pub fn take_progress_item(self) -> Option<T>
    {

        if let Self::InProgress(val) = self
        {

            Some(val)

        }
        else
        {

            None
            
        }

    }

    pub fn is_ended(&self) -> bool
    {

        matches!(self, Self::Ended)

    }

}

impl<T> Clone for BasicStatusWithItem<T>
    where T: Clone
{

    fn clone(&self) -> Self
    {

        match self
        {

            Self::InProgress(arg0) => Self::InProgress(arg0.clone()),
            Self::Ended => Self::Ended,

        }

    }

}

impl<T> BasicStatusWithItem<T>
    where T: Clone
{

    pub fn clone_progress_item(&self) -> Option<T>
    {

        if let Self::InProgress(val) = self
        {

            Some(val.clone())

        }
        else
        {

            None
            
        }

    }

}

impl<T> Debug for BasicStatusWithItem<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InProgress(arg0) => f.debug_tuple("InProgress").field(arg0).finish(),
            Self::Ended => write!(f, "Ended"),
        }
    }

}