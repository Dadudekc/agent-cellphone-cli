pub fn run(input: &str) -> String {
    format!("exec: {}", input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        assert_eq!(run("x"), "exec: x");
    }
}
