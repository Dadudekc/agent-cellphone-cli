pub mod monolith;
pub mod dedup;
pub mod ssot;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modules() {
        assert!(monolith::check());
        assert!(dedup::check());
        assert!(ssot::check());
    }
}
