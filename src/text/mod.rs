//!
//! Send text between threads. 
//! 

mod sendable_text;

pub use sendable_text::*;

mod sendable_text_immut;

pub use sendable_text_immut::*;

#[cfg(feature = "corlib")]
mod sendable_text_log;

#[cfg(feature = "corlib")]
pub use sendable_text_log::*;

#[cfg(feature = "corlib")]
mod sendable_text_log_with_buffer;

#[cfg(feature = "corlib")]
pub use sendable_text_log_with_buffer::*;


