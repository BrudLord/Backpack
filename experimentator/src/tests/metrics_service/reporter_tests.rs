#[cfg(test)]
mod tests {
    use crate::metrics_service::reporter::Reporter;

    use super::*;
    use serde::Serialize;
    use std::fs::{self, File};
    use std::io::{self, Read};
    use std::process::Output;
    use tempfile::NamedTempFile;

    #[derive(Serialize)]
    struct TestData {
        name: String,
        value: i32,
    }

    fn create_test_data() -> TestData {
        TestData {
            name: "test".to_string(),
            value: 42,
        }
    }

    fn read_file_content(path: &str) -> String {
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn test_console_reporter_creation() {
        let reporter = Reporter::new(None);
        assert!(reporter.is_ok());
    }

    #[test]
    fn test_file_reporter_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let reporter = Reporter::new(Some(temp_file.path().to_str().unwrap()));
        assert!(reporter.is_ok());
    }

    #[test]
    fn test_file_reporter_write() -> io::Result<()> {
        // Setup
        let temp_file = NamedTempFile::new()?;
        let reporter = Reporter::new(Some(temp_file.path().to_str().unwrap()))?;
        let test_message = "Test message";

        // Test
        reporter.report(&test_message)?;

        // Verify
        let content = read_file_content(temp_file.path().to_str().unwrap());
        assert_eq!(content, "Test message\n");
        Ok(())
    }

    #[test]
    fn test_json_report() -> io::Result<()> {
        // Setup
        let temp_file = NamedTempFile::new()?;
        let reporter = Reporter::new(Some(temp_file.path().to_str().unwrap()))?;
        let test_data = create_test_data();

        // Test
        reporter.report_json(&test_data)?;

        // Verify
        let content = read_file_content(temp_file.path().to_str().unwrap());
        let expected = serde_json::to_string(&test_data)? + "\n";
        assert_eq!(content, expected);
        Ok(())
    }

    #[test]
    fn test_batch_report() -> io::Result<()> {
        // Setup
        let temp_file = NamedTempFile::new()?;
        let reporter = Reporter::new(Some(temp_file.path().to_str().unwrap()))?;
        let test_data = vec![
            create_test_data(),
            TestData {
                name: "test2".to_string(),
                value: 43,
            },
        ];

        // Test
        reporter.report_batch(&test_data)?;

        // Verify
        let content = read_file_content(temp_file.path().to_str().unwrap());
        let expected = test_data.iter()
            .map(|data| serde_json::to_string(data).unwrap() + "\n")
            .collect::<String>();
        assert_eq!(content, expected);
        Ok(())
    }

    #[test]
    fn test_invalid_file_path() {
        let reporter = Reporter::new(Some("/invalid/path/file.txt"));
        assert!(reporter.is_err());
    }

    #[test]
    fn test_concurrent_writes() -> io::Result<()> {
        use std::sync::Arc;
        use std::thread;

        // Setup
        let temp_file = NamedTempFile::new()?;
        let reporter = Arc::new(Reporter::new(Some(temp_file.path().to_str().unwrap()))?);
        let thread_count = 10;
        let mut handles = vec![];

        // Test concurrent writes
        for i in 0..thread_count {
            let reporter_clone = Arc::clone(&reporter);
            let handle = thread::spawn(move || {
                reporter_clone.report(&format!("Thread {}", i)).unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify
        let content = read_file_content(temp_file.path().to_str().unwrap());
        assert_eq!(content.lines().count(), thread_count);
        Ok(())
    }

    #[test]
    fn test_large_data() -> io::Result<()> {
        // Setup
        let temp_file = NamedTempFile::new()?;
        let reporter = Reporter::new(Some(temp_file.path().to_str().unwrap()))?;
        let large_string = "a".repeat(1_000_000);

        // Test
        reporter.report(&large_string)?;

        // Verify
        let content = read_file_content(temp_file.path().to_str().unwrap());
        assert_eq!(content, large_string + "\n");
        Ok(())
    }
}