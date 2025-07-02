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
pub struct WorkInProgressResult<T>
{

    result: Option<T>,
    done: bool

}

impl<T> WorkInProgressResult<T>
{

    pub fn new(result: Option<T>, done: bool) -> Self
    {

        Self
        {

            result,
            done

        }

    }

    pub fn done(result: T) -> Self
    {

        Self::new(Some(result), true)

    }

    pub fn opt_done(result: Option<T>) -> Self
    {

        Self::new(result, true)

    }

    pub fn not_done(result: T) -> Self
    {

        Self::new(Some(result), false)

    }

    pub fn opt_not_done(result: Option<T>) -> Self
    {

        Self::new(result, false)

    }

    pub fn none_done() -> Self
    {

        Self::new(None, true)

    }

    pub fn none_not_done() -> Self
    {

        Self::new(None, false)

    }

    pub fn result_ref(&self) -> &Option<T>
    {

        &self.result

    }

    pub fn is_done(&self) -> bool
    {

        self.done

    }

    pub fn take_result(self) -> Option<T>
    {

        self.result

    }

}

impl<T> Display for WorkInProgressResult<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match &self.result
        {

            Some(val) =>
            {

                write!(f, "{{ result: {0}, done: {1} }}", val, self.done)

            }
            None =>
            {

                write!(f, "{{ done: {0} }}", self.done)

            }

        } 
        
    }

}

impl<T> Debug for WorkInProgressResult<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkInProgressResult").field("result", &self.result).field("done", &self.done).finish()
    }

}

impl<T> Clone for WorkInProgressResult<T>
    where T: Clone
{

    fn clone(&self) -> Self {
        Self { result: self.result.clone(), done: self.done.clone() }
    }

}

impl<T> Default for WorkInProgressResult<T>
    where T: Default
{

    fn default() -> Self {
        Self { result: Default::default(), done: Default::default() }
    }

}

/*
#[cfg(feature = "serde")]
impl<T> Serialize for WorkInProgressResult<T>
    where T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {

        let mut state = serializer.serialize_struct("WorkInProgressResult", 2)?;

        state.serialize_field("result", &self.result)?;

        state.serialize_field("done", &self.done)?;

        state.end()

        
    }

}

struct WorkInProgressResultVisitor;

impl<'de> Visitor<'de> for WorkInProgressResultVisitor
{

    type Value = SendableText;

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

impl<'de> Deserialize<'de> for SendableText
{

    fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {

        let visitor = SendableTextVisitor{};

        deserialiser.deserialize_string(visitor)
    
    }

}
*/

//#[cfg_attr(feature = "serde", Serialize, Deserialize)]

///
/// A WorkInProgressResult with an id.
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdedWorkInProgressResult<ID, T>
{

    id: ID,
    work_in_progress_result: WorkInProgressResult<T>

}

impl<ID, T> IdedWorkInProgressResult<ID, T>
{

    pub fn new(id: ID, result: Option<T>, done: bool) -> Self
    {

        Self
        {

            id,
            work_in_progress_result: WorkInProgressResult::new(result, done)

        }

    }

    pub fn id(&self) -> &ID
    {

        &self.id

    }

    pub fn done(id: ID, result: T) -> Self
    {

        Self::new(id, Some(result), true)

    }

    pub fn opt_done(id: ID, result: Option<T>) -> Self
    {

        Self::new(id, result, true)

    }

    pub fn not_done(id: ID, result: T) -> Self
    {

        Self::new(id, Some(result), false)

    }

    pub fn opt_not_done(id: ID, result: Option<T>) -> Self
    {

        Self::new(id, result, false)

    }

    pub fn none_done(id: ID) -> Self
    {

        Self::new(id, None, true)

    }

    pub fn none_not_done(id: ID) -> Self
    {

        Self::new(id, None, false)

    }

    pub fn take_id_and_result(self) -> (ID, Option<T>)
    {

        (self.id, self.work_in_progress_result.result)

    }
    
    delegate!
    {

        to self.work_in_progress_result
        {

            pub fn result_ref(&self) -> &Option<T>;

            pub fn is_done(&self) -> bool;

            pub fn take_result(self) -> Option<T>;

        }

    }

}

impl<ID, T> Display for IdedWorkInProgressResult<ID, T>
    where ID: Display, T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match &self.work_in_progress_result.result
        {

            Some(val) =>
            {

                write!(f, "{{ Id: {0}, result: {1}, done: {2} }}", self.id, val, self.work_in_progress_result.done)

            }
            None =>
            {

                write!(f, "{{ Id: {0}, done: {1} }}", self.id, self.work_in_progress_result.done)

            }

        } 
        
    }

}

impl<ID, T> Debug for IdedWorkInProgressResult<ID, T>
    where ID: Debug, T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdedWorkInProgressResult").field("id", &self.id).field("work_in_progress_result", &self.work_in_progress_result).finish()
    }

}

impl<ID, T> Clone for IdedWorkInProgressResult<ID, T>
    where ID: Clone, T: Clone
{

    fn clone(&self) -> Self {
        Self { id: self.id.clone(), work_in_progress_result: self.work_in_progress_result.clone() }
    }

}

impl<ID, T> Default for IdedWorkInProgressResult<ID, T>
    where ID: Default, T: Default
{

    fn default() -> Self {
        Self { id: Default::default(), work_in_progress_result: Default::default() }
    }

}

