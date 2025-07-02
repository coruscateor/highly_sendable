#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//Disabled - to be removed

//pub mod result;

pub mod text;

mod work_in_progress_result;

pub use work_in_progress_result::*;

mod work_in_progress_result_int_percentage;

pub use work_in_progress_result_int_percentage::*;

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
