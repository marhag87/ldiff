/// Compares two strings line by line.
/// Matching lines are shown with =
/// Non-matching lines are shown with |
/// ```
/// use ldiff::compare;
/// let left = "asdf".to_string();
/// let right = "asdf".to_string();
/// let expected = "asdf = asdf\n".to_string();
/// let result = compare(left, right);
///
/// assert_eq!(expected, result);
/// ```
pub fn compare(left_content: String, right_content: String) -> String {
    let left_lines: Vec<&str> = left_content.trim().split('\n').collect();
    let right_lines: Vec<&str> = right_content.trim().split('\n').collect();

    let max = max(&left_lines);
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

    let left_len = left_lines.len();
    let right_len = right_lines.len();
    if left_len > right_len {
        for x in right_len..left_len {
            result += &format!("{:width$} |\n", left_lines[x], width = max);
        }
    } else if right_len > left_len {
        for x in left_len..right_len {
            result += &format!("{:width$} | {}\n", "", right_lines[x], width = max);
        }
    }

    result
}

/// Get max length of &strs in a &Vec
/// ```
/// use ldiff::max;
/// let strings = vec!["a", "ab", "abc", "d"];
/// assert_eq!(3, max(&strings));
/// ```
pub fn max(list: &Vec<&str>) -> usize {
    list.iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len()
}

#[test]
fn right_short() {
    let left = "asd\n123\n".to_string();
    let right = "asd".to_string();
    let expected = "asd = asd\n123 |\n".to_string();
    let actual = compare(left, right);
    assert_eq!(expected, actual);
}

#[test]
fn left_short() {
    let left = "asd".to_string();
    let right = "asd\n123\n".to_string();
    let expected = "asd = asd\n    | 123\n".to_string();
    let actual = compare(left, right);
    assert_eq!(expected, actual);
}

#[test]
fn fill_long_left() {
    let left = "asdasdasdasdasdasd".to_string();
    let right = "asd".to_string();
    let expected = "asdasdasdasdasdasd | asd\n".to_string();
    let actual = compare(left, right);
    assert_eq!(expected, actual);
}

#[test]
fn fill_long_right() {
    let left = "asd".to_string();
    let right = "asdasdasdasdasdasd".to_string();
    let expected = "asd | asdasdasdasdasdasd\n".to_string();
    let actual = compare(left, right);
    assert_eq!(expected, actual);
}
