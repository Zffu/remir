use std::fmt::Display;

#[inline]
pub fn fmt_list<K: Display>(list: &[K]) -> String {
    let mut str = format!("{}", list[0]);

    for i in 1..list.len() {
        str += &format!(", {}", list[i]);
    }

    str
}
