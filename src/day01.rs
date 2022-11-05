/// Count the number of times a measurement increases in the given slice.
///
/// # Examples
///
/// ```
/// use aoc21::day01::num_larger_measurements;
///
/// assert_eq!(7, num_larger_measurements(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263].as_slice()));
/// ```
pub fn num_larger_measurements(ms: &[i64]) -> i64 {
    ms.iter()
        .fold((i64::MAX, 0_i64), |(last, count), &i| {
            (i, if i > last { count + 1 } else { count })
        })
        .1
}
