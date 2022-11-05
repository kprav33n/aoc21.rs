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

/// Count the number of times a window measurement increases in the given slice.
///
/// # Examples
///
/// ```
/// use aoc21::day01::num_larger_measurement_windows;
///
/// assert_eq!(5, num_larger_measurement_windows(vec![607, 618, 618, 617, 647, 716, 769, 792].as_slice()));
/// ```
pub fn num_larger_measurement_windows(ms: &[i64]) -> i64 {
    let mut result = 0;
    let mut last = 0;
    for i in 1..ms.len() - 2 {
        let sum = ms[i] + ms[i + 1] + ms[i + 2];
        if sum > last {
            result += 1;
        }
        last = sum;
    }
    result
}
