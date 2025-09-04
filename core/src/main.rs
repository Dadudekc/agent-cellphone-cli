mod exec;
mod edits;
mod locks;
mod telemetry;
mod gates;

fn main() {
    println!("{}", exec::run("hello"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main_runs() {
        exec::run("test");
    }
}
