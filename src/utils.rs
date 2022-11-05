/// Splits the given string on line breaks and return a vector of integers.
///
/// # Examples
///
/// ```
/// use aoc21::utils::lines_as_i64_vec;
///
/// assert_eq!(lines_as_i64_vec("12\n39\n64"), vec![12, 39, 64]);
/// assert_eq!(lines_as_i64_vec("\n12\n39\n64\n"), vec![12, 39, 64]);
/// ```
pub fn lines_as_i64_vec(s: &str) -> Vec<i64> {
    s.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
