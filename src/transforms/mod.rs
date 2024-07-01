pub fn create_safe_name(input: &str) -> String {
    input
        .chars()
        .map(|c| match c.is_whitespace() {
            true => '-',
            false => c,
        })
        .filter(|c| c.is_ascii())
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_safe_name() {
        assert_eq!(
            create_safe_name("Link's 🗡️ is called マスターソード"),
            "links--is-called-"
        )
    }
}
