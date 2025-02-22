pub fn condense(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}
