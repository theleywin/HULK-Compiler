use std::collections::HashSet;

/// Converts a `HashSet<usize>` to a sorted space-separated `String` representation.
///
/// # Arguments
///
/// * `s` - A reference to a `HashSet<usize>`
///
/// # Returns
///
/// A `String` containing the sorted elements separated by spaces.
///
pub fn to_str(s: &HashSet<usize>) -> String {
    let mut v: Vec<usize> = s.iter().cloned().collect();
    v.sort_unstable();
    let repr: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    repr.join(" ")
}

/// Converts a space-separated string of numbers into a `HashSet<usize>`.
///
/// # Arguments
///
/// * `s` - A reference to a `String` containing space-separated numbers.
///
/// # Returns
///
/// A `HashSet<usize>` containing the parsed numbers.
///
/// # Panics
///
/// Panics if any of the substrings cannot be parsed into a `usize`.
///
pub fn to_set(s: &String) -> HashSet<usize> {
    let nums: HashSet<usize> = s.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    nums
}