//!
//! Send text between threads. 
//! 

mod sendable_text;

#[cfg(feature = "corlib")]
pub use sendable_text::*;

#[cfg(feature = "corlib")]
mod sendable_text_log;

#[cfg(feature = "corlib")]
pub use sendable_text_log::*;

#[cfg(feature = "corlib")]
mod sendable_text_log_with_buffer;

#[cfg(feature = "corlib")]
pub use sendable_text_log_with_buffer::*;


