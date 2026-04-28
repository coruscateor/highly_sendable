
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum BasicStatusU32
{

    InProgress(u32),
    Ended

}

impl BasicStatusU32
{

    pub fn is_in_progress(&self) -> bool
    {

        matches!(self, Self::InProgress(_))

    }

    pub fn in_progress_number(&self) -> Option<u32>
    {

        if let Self::InProgress(val) = self
        {

            Some(*val)

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