//! Formatting utilties

use std::fmt::Display;

/// Formats a list of [`Display`] elements with the given form:
/// `0, 1, 2, 3, 4, ..`
///
/// # Example
/// ```
/// use crate::fmt::utils::fmt_list;
///
/// let myList: Vec<String> = vec!["hello".to_string(), "world".to_string()];
/// assert_eq(&fmt_list(&myList), "hello, world");
/// ```
///
#[inline]
pub fn fmt_list<K: Display>(list: &[K]) -> String {
    let mut str = format!("{}", list[0]);

    for i in 1..list.len() {
        str += &format!(", {}", list[i]);
    }

    str
}
