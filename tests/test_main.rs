#[cfg(test)]
mod tests {
    use serde_yml::run;

    #[test]
    fn test_run_success() {
        // Test that run() executes without panicking
        assert!(run().is_ok());
    }
}
