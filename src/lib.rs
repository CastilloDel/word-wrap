pub fn wrap_lines(lines: &str, limit: usize) -> String {
    if lines.len() <= limit {
        return lines.to_string();
    }
    let space_before_limit = lines[0..limit].rfind(" ");
    let line = &lines[0..(space_before_limit.unwrap_or(limit))];
    let rest = &lines[space_before_limit.map(|val| val + 1).unwrap_or(limit)..];

    format!("{}\n{}", line, &wrap_lines(rest.trim(), limit),)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_change_a_lines_if_it_is_not_larger_than_the_limit() {
        assert_eq!("", wrap_lines("", 10));
        assert_eq!("123", wrap_lines("123", 3));
    }

    #[test]
    fn should_break_if_the_line_is_larger_than_the_limit() {
        assert_eq!("123\n45", wrap_lines("12345", 3));
        assert_eq!("123\n456\n7", wrap_lines("1234567", 3));
    }

    #[test]
    fn should_prefer_breaking_at_existing_spaces() {
        assert_eq!("123\n4578\n909", wrap_lines("123 4578909", 4));
    }

    #[test]
    fn should_ignore_spaces_if_not_inside_a_line() {
        assert_eq!("123\n4578\n909", wrap_lines("123  4578   909    ", 4));
    }
}
