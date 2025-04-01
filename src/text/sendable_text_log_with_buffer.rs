use corlib::collections::Queue;

use delegate::delegate;

use super::{SendableText, SendableTextLog};

static NO_STRING_ERROR_MESSAGE: &str = "Error: There has to be a String here.";

///
/// Maintains a Queue of SendableText objects to be copied into an internal buffer.
/// 
pub struct SendableTextLogWithBuffer
{

    stl: SendableTextLog,
    buffer: Option<String>

}

impl SendableTextLogWithBuffer
{

    pub fn new(limit: usize) -> Self
    {

        Self
        {

            stl: SendableTextLog::new(limit),
            buffer: Some(String::new())

        }

    }
    
    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacity(capacity),
            buffer: Some(String::new())

        }

    }

    pub fn with_capacit_and_limit(capacity: usize, limit: usize) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacit_and_limit(capacity, limit),
            buffer: Some(String::new())

        }

    }

    //buffer

    pub fn with_buffer(limit: usize, buffer: String) -> Self
    {

        Self
        {

            stl: SendableTextLog::new(limit),
            buffer: Some(buffer)

        }

    }
    
    pub fn with_capacity_and_buffer(capacity: usize, buffer: String) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacity(capacity),
            buffer: Some(buffer)

        }

    }

    pub fn with_capaciy_limit_and_buffer(capacity: usize, limit: usize, buffer: String) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacit_and_limit(capacity, limit),
            buffer: Some(buffer)

        }

    }

    //

    delegate! {
        to self.stl {

            pub fn capacity(&self) -> usize;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn limit(&self) -> usize;

            pub fn set_limit(&mut self, new_limit: usize);

            pub fn append_to(&self, output: &mut String);

            pub fn overwrite(&self, output: &mut String);

            #[call(push)]
            pub fn push_only(&mut self, st: SendableText);

        }
    }

    pub fn buffer_ref(&self) -> &String
    {

        self.buffer.as_ref().expect(NO_STRING_ERROR_MESSAGE)

    }

    pub fn push(&mut self, st: SendableText)
    {

        self.stl.push(st);

        self.overwite_buffer();

    }

    pub fn clear(&mut self)
    {

        self.stl.clear();

        let mut_buffer = self.buffer.as_mut().expect(NO_STRING_ERROR_MESSAGE);
        
        mut_buffer.clear();

    }

    pub fn overwite_buffer(&mut self)
    {

        let mut res = self.buffer.take().expect(NO_STRING_ERROR_MESSAGE);

        self.overwrite(&mut res);

        self.buffer = Some(res);

    }

}

