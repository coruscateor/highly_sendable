use std::fmt::{write, Display, Formatter};

use delegate::delegate;

//use serde::{de::Visitor, ser::{Error, SerializeStruct}};

use std::fmt::Debug;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

///
/// For indicating when some work has been done and/or is still in the process of being done.
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkInProgressResultIntPercentage<T>
{

    result: Option<T>,
    progress: u8

}

impl<T> WorkInProgressResultIntPercentage<T>
{

    pub fn new(result: Option<T>, progress: u8) -> Self
    {
        
        let mut checked_progress = progress;

        if progress < 100
        {

            checked_progress = 100;

        }

        Self
        {

            result,
            progress: checked_progress

        }

    }
    
    /*
    pub fn with_progress(result: Option<T>, progress: u8) -> Self
    {

        let mut checked_progress = progress;

        if progress < 100
        {

            checked_progress = 100;

        }

        Self
        {

            result,
            progress: checked_progress

        }

    }
    */

    pub fn done(result: T) -> Self
    {

        Self::new(Some(result), 100)

    }

    pub fn opt_done(result: Option<T>) -> Self
    {

        Self::new(result, 100)

    }

    pub fn result_progress(result: T, progress: u8) -> Self
    {

        Self::new(Some(result), progress)

    }

    pub fn result_no_progress(result: T) -> Self
    {

        Self::new(Some(result), 0)

    }

    pub fn opt_no_progress(result: Option<T>) -> Self
    {

        Self::new(result, 0)

    }

    pub fn none_done() -> Self
    {

        Self::new(None, 100)

    }

    pub fn none_no_progress() -> Self
    {

        Self::new(None, 0)

    }

    pub fn result_ref(&self) -> &Option<T>
    {

        &self.result

    }

    pub fn progress(&self) -> u8
    {

        self.progress

    }

    pub fn is_done(&self) -> bool
    {

        self.progress == 100

    }

    pub fn no_progress(&self) -> bool
    {

        self.progress == 0

    }

    pub fn take_result(self) -> Option<T>
    {

        self.result

    }

}

impl<T> Display for WorkInProgressResultIntPercentage<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match &self.result
        {

            Some(val) =>
            {

                write!(f, "{{ result: {0}, progress: {1} }}", val, self.progress)

            }
            None =>
            {

                write!(f, "{{ progress: {0} }}", self.progress)

            }

        } 
        
    }

}

impl<T> Debug for WorkInProgressResultIntPercentage<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkInProgressResultIntPercentage").field("result", &self.result).field("progress", &self.progress).finish()
    }

}

impl<T> Clone for WorkInProgressResultIntPercentage<T>
    where T: Clone
{

    fn clone(&self) -> Self {
        Self { result: self.result.clone(), progress: self.progress.clone() }
    }

}

impl<T> Default for WorkInProgressResultIntPercentage<T>
    where T: Default
{

    fn default() -> Self {
        Self { result: Default::default(), progress: Default::default() }
    }

}

//
/// A WorkInProgressResultIntPercentage with an id.
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdedWorkInProgressResultIntPercentage<ID, T>
{

    id: ID,
    work_in_progress_result_int_percentage: WorkInProgressResultIntPercentage<T>

}

impl<ID, T> IdedWorkInProgressResultIntPercentage<ID, T>
{

    pub fn new(id: ID, result: Option<T>, progress: u8) -> Self
    {

        Self
        {

            id,
            work_in_progress_result_int_percentage: WorkInProgressResultIntPercentage::new(result, progress)

        }

    }

    /* 
    pub fn with_progress(id: ID, result: Option<T>, progress: u8) -> Self
    {

        Self
        {

            id,
            work_in_progress_result_int_percentage: WorkInProgressResultIntPercentage::with_progress(result, progress)

        }

    }
    */

    pub fn id_ref(&self) -> &ID
    {

        &self.id

    }

    pub fn done(id: ID, result: T) -> Self
    {

         Self::new(id, Some(result), 100)

    }

    pub fn opt_done(id: ID, result: Option<T>) -> Self
    {

        Self::new(id, result, 100)

    }

    pub fn result_progress(id: ID, result: T, progress: u8) -> Self
    {

        Self::new(id, Some(result), progress)

    }

    pub fn result_no_progress(id: ID, result: T) -> Self
    {

        Self::new(id,Some(result), 0)

    }

    pub fn opt_no_progress(id: ID, result: Option<T>) -> Self
    {

        Self::new(id, result, 0)

    }

    pub fn none_done(id: ID) -> Self
    {

        Self::new(id, None, 100)

    }

    pub fn none_no_progress(id: ID) -> Self
    {

        Self::new(id, None, 0)

    }

    pub fn take_id_and_result(self) -> (ID, Option<T>)
    {

        (self.id, self.work_in_progress_result_int_percentage.result)

    }
    
    delegate!
    {

        to self.work_in_progress_result_int_percentage
        {

            pub fn result_ref(&self) -> &Option<T>;

            pub fn progress(&self) -> u8;

            pub fn is_done(&self) -> bool;

            pub fn no_progress(&self) -> bool;

            pub fn take_result(self) -> Option<T>;

        }

    }

}

impl<ID, T> Display for IdedWorkInProgressResultIntPercentage<ID, T>
    where ID: Display, T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match &self.work_in_progress_result_int_percentage.result
        {

            Some(val) =>
            {

                write!(f, "{{ Id: {0}, result: {1}, progress: {2} }}", self.id, val, self.work_in_progress_result_int_percentage.progress)

            }
            None =>
            {

                write!(f, "{{ Id: {0}, progress: {1} }}", self.id, self.work_in_progress_result_int_percentage.progress)

            }

        } 
        
    }

}

impl<ID, T> Debug for IdedWorkInProgressResultIntPercentage<ID, T>
    where ID: Debug, T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdedWorkInProgressResultIntPercentage").field("id", &self.id).field("work_in_progress_result_int_percentage", &self.work_in_progress_result_int_percentage).finish()
    }

}

impl<ID, T> Clone for IdedWorkInProgressResultIntPercentage<ID, T>
    where ID: Clone, T: Clone
{

    fn clone(&self) -> Self {
        Self { id: self.id.clone(), work_in_progress_result_int_percentage: self.work_in_progress_result_int_percentage.clone() }
    }

}

impl<ID, T> Default for IdedWorkInProgressResultIntPercentage<ID, T>
    where ID: Default, T: Default
{

    fn default() -> Self {
        Self { id: Default::default(), work_in_progress_result_int_percentage: Default::default() }
    }

}

