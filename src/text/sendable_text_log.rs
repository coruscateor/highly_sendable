use corlib::collections::Queue;

use delegate::delegate;

use super::SendableText;

///
/// Maintains a Queue of SendableText objects to be copied into a provided buffer.
/// 
pub struct SendableTextLog
{

    st_queue: Queue<SendableText>,
    limit: usize

}

impl SendableTextLog
{

    pub fn new(limit: usize) -> Self
    {

        Self
        {

            st_queue: Queue::new(),
            limit

        }

    }
    
    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            st_queue: Queue::with_capacity(capacity),
            limit: capacity

        }

    }

    pub fn with_capacit_and_limit(capacity: usize, limit: usize) -> Self
    {

        Self
        {

            st_queue: Queue::with_capacity(capacity),
            limit

        }

    }

    pub fn limit(&self) -> usize
    {

        self.limit

    }

    pub fn set_limit(&mut self, new_limit: usize)
    {

        self.limit = new_limit;

    }

    delegate! {
        to self.st_queue {

            pub fn capacity(&self) -> usize;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn clear(&mut self); 

        }
    }

    pub fn push(&mut self, st: SendableText)
    {

        if self.st_queue.len() == self.limit
        {

            self.st_queue.pop();

        }

        self.st_queue.push(st);

    }

    //https://doc.rust-lang.org/std/collections/vec_deque/struct.Iter.html

    pub fn append_to(&self, output: &mut String)
    {

        for item in self.st_queue.iter().rev()
        {

            output.push_str(item);

        }

    }

    pub fn overwrite(&self, output: &mut String)
    {

        output.clear();

        self.append_to(output);

    }

}


