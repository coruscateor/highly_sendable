
pub trait VecBufferInputOutput
    where Self: Send + Sync
{
    
    fn put_buffer(&self, buffer: Vec<u8>);

    fn put_opt_buffer(&self, opt_buffer: Option<Vec<u8>>);

    fn take_buffer(&self) -> Vec<u8>;

    fn take_buffer_with_capacity(&self, capacity: usize) -> Vec<u8>;

}

pub struct EmptyVecBufferInputOutput();

impl VecBufferInputOutput for EmptyVecBufferInputOutput
{

    fn put_buffer(&self, _buffer: Vec<u8>)
    {
    }

    fn put_opt_buffer(&self, _opt_buffer: Option<Vec<u8>>)
    {
    }

    fn take_buffer(&self) -> Vec<u8>
    {

        Vec::new()

    }
    
    fn take_buffer_with_capacity(&self, capacity: usize) -> Vec<u8>
    {
        
        Vec::with_capacity(capacity)

    }

}