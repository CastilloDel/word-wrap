pub fn wrap_lines(lines: &str, limit: usize) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_not_change_a_lines_if_it_is_not_larger_than_the_limit() {
        assert_eq!("", wrap_lines("", 10));
        assert_eq!("abc", wrap_lines("abc", 10));
    }
}
