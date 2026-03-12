
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

        matches!(self, EssentialStatus::InProgress)

    }

    pub fn is_not_in_progress(&self) -> bool
    {

        matches!(self, EssentialStatus::NotInProgress)

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

        matches!(self, PauseableStatus::InProgress)

    }

    pub fn is_paused(&self) -> bool
    {

        matches!(self, PauseableStatus::Paused)

    }

    pub fn is_ended(&self) -> bool
    {

        matches!(self, PauseableStatus::Ended)

    }

}