pub fn wrap_lines(lines: &str, limit: usize) -> String {
    lines
        .split("\n")
        .map(|line| wrap_line(line, limit))
        .reduce(|a, b| format!("{a}\n{b}"))
        .unwrap()
}

fn wrap_line(line: &str, limit: usize) -> String {
    if line.len() <= limit {
        return line.to_string();
    }
    let space_before_limit = line[0..limit].rfind(" ");
    let first_slice = &line[0..(space_before_limit.unwrap_or(limit))];
    let rest = &line[space_before_limit.map(|val| val + 1).unwrap_or(limit)..];

    format!("{}\n{}", first_slice, &wrap_lines(rest.trim(), limit),)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_change_a_line_if_it_is_not_larger_than_the_limit() {
        assert_eq!("", wrap_lines("", 10));
        assert_eq!("1234\n56", wrap_lines("1234\n56", 6));
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
