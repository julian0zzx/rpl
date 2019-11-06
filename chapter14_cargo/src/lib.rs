
//! # crate: chapter_cargo
//! test
//! 
//! (special hashtag <b># Example</b>, belowing code will be run during test phase)

/// # Example add_one, 
/// ```
/// let r = 13;
/// assert_eq!(13, chapter14_cargo::add_one(12));
/// ```
pub fn add_one(input : i64) -> i64 {
    input + 1
}
