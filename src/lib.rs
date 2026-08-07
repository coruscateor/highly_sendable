#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod text;

/*

mod work_in_progress_result;

pub use work_in_progress_result::*;

mod work_in_progress_result_int_percentage;

pub use work_in_progress_result_int_percentage::*;

mod work_in_progress_option;

pub use work_in_progress_option::*;

mod  work_in_progress_option_int_percentage;

pub use work_in_progress_option_int_percentage::*;

 */

mod work_in_progress_message;

pub use work_in_progress_message::*;

mod status_definitions;

pub use status_definitions::*;

mod variable_state_number;

pub use variable_state_number::*;

mod variable_state_message;

pub use variable_state_message::*;

//Disabled

//mod connection_state_id;

//pub use connection_state_id::*;

//mod connection_message;

//pub use connection_message::*;

mod sendable_bytes;

pub use sendable_bytes::*;

mod sendable_ref;

pub use sendable_ref::*;

mod cowable_ref;

pub use cowable_ref::*;

mod buffer_retrival;

pub use buffer_retrival::*;
