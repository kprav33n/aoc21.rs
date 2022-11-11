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
    lines_as_vec(s, |s| s.parse::<i64>().unwrap())
}

/// Splits the given string on line breaks and return a vector of given type.
pub fn lines_as_vec<T>(s: &str, f: fn(&str) -> T) -> Vec<T> {
    s.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(f)
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_lines_as_vec() {
        assert_eq!(
            lines_as_vec("12\n39\n64", |s| s.parse::<i64>().unwrap()),
            vec![12, 39, 64]
        );
        assert_eq!(
            lines_as_vec("\n12\n39\n64\n", |s| s.parse::<i64>().unwrap()),
            vec![12, 39, 64]
        );
    }
}
