/// Compares two strings line by line.
/// Matching lines are shown with =
/// Non-matching lines are shown with |
/// ```
/// use ldiff::compare;
/// let left = "asdf".to_string();
///    let right = "asdf".to_string();
///    let expected = "asdf = asdf\n".to_string();
///    let result = compare(left, right);
///
///    assert_eq!(expected, result);
/// ```
pub fn compare(left_content: String, right_content: String) -> String {
    let left_lines: Vec<&str> = left_content.trim().split('\n').collect();
    let right_lines: Vec<&str> = right_content.trim().split('\n').collect();

    let max = left_lines
        .iter()
        .chain(right_lines.iter())
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();

    let mut result = String::new();
    left_lines
        .iter()
        .zip(right_lines.iter())
        .for_each(|(left, right)| {
            let mut separator = '|';
            if left == right {
                separator = '=';
            }
            result += &format!("{:width$} {} {}\n", left, separator, right, width = max);
        });

    result
}
