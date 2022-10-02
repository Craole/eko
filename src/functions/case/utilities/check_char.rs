pub fn is_digit(c: &str) -> bool {
    c.chars().all(|c| c.is_ascii_digit())
}

pub fn is_uppercase(c: &str) -> bool {
    c.to_uppercase() != c.to_lowercase() && c == c.to_uppercase()
}

pub fn is_lowercase(c: &str) -> bool {
    c.to_uppercase() != c.to_lowercase() && c == c.to_lowercase()
}
