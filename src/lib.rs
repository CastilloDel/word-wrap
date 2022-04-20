pub fn wrap_lines(lines: &str, limit: usize) -> String {
    if lines.len() <= limit {
        return lines.to_string();
    }
    let space_before_limit = lines[0..limit].rfind(" ");
    let (break_index, next_start) = match space_before_limit {
        Some(val) => (val, val + 1),
        None => (limit, limit),
    };
    format!(
        "{}\n{}",
        &lines[0..break_index],
        &wrap_lines(&lines[next_start..], limit),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_change_a_lines_if_it_is_not_larger_than_the_limit() {
        assert_eq!("", wrap_lines("", 10));
        assert_eq!("123", wrap_lines("123", 10));
        assert_eq!("123", wrap_lines("123", 3));
    }

    #[test]
    fn should_break_if_the_line_is_larger_than_the_limit() {
        assert_eq!("123\n45", wrap_lines("12345", 3));
        assert_eq!("123\n456\n7", wrap_lines("1234567", 3));
    }

    #[test]
    fn should_prefer_breaking_in_existing_spaces() {
        assert_eq!("123\n45", wrap_lines("123 45", 4));
        assert_eq!("123\n4578\n909", wrap_lines("123 4578909", 4));
    }
}
