pub fn check() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check() {
        assert!(check());
    }
}
