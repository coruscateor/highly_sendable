#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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

mod connection_state_id;

pub use connection_state_id::*;

mod connection_message;

pub use connection_message::*;

/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
