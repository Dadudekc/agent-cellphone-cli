pub fn record(event: &str) -> String {
    format!("recorded: {event}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_record() {
        assert_eq!(record("e"), "recorded: e");
    }
}
