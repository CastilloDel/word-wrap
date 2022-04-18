pub fn wrap_lines(lines: &str, limit: usize) -> String {
    if lines.len() > limit {
        format!(
            "{}\n{}",
            &lines[0..limit],
            &wrap_lines(&lines[limit..], limit)
        )
    } else {
        lines.to_string()
    }
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
}
