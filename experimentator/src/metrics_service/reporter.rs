use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

/// A reporter that handles output to either a file or console.
///
/// This struct provides methods to write formatted data, JSON, and batches of data
/// to either a file or standard output.
pub struct Reporter {
    output: Mutex<Output>,
}

/// Represents the output destination for the reporter.
enum Output {
    /// File output with an open file handle
    File(File),
    /// Standard console output
    Console,
}

impl Reporter {
    /// Creates a new Reporter instance.
    ///
    /// # Arguments
    ///
    /// * `file_path` - An optional file path. If provided, output will be written to the file.
    ///                 If None, output will be written to console.
    ///
    /// # Returns
    ///
    /// * `Result<Reporter, io::Error>` - A Result containing the Reporter if successful,
    ///   or an IO error if file creation fails.
    pub fn new(file_path: Option<&str>) -> io::Result<Self> {
        let output = match file_path {
            Some(path) => Output::File(File::create(Path::new(path))?),
            None => Output::Console,
        };

        Ok(Self {
            output: Mutex::new(output),
        })
    }

    /// Reports formatted data to the output destination.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be reported, must implement Display trait.
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - Ok if writing succeeds, Err if it fails.
    pub fn report<T: std::fmt::Display>(&self, data: &T) -> io::Result<()> {
        let output = format!("{}\n", data);
        let mut guard = self.output.lock().unwrap();
        
        match &mut *guard {
            Output::File(file) => file.write_all(output.as_bytes()),
            Output::Console => io::stdout().write_all(output.as_bytes()),
        }
    }

    /// Reports data in JSON format to the output destination.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be serialized to JSON and reported.
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - Ok if serialization and writing succeed, Err if either fails.
    pub fn report_json<T: Serialize>(&self, data: &T) -> io::Result<()> {
        let json = serde_json::to_string(data)?;
        self.report(&json)
    }

    /// Reports a batch of items in JSON format to the output destination.
    ///
    /// # Arguments
    ///
    /// * `items` - A slice of items to be reported in JSON format.
    ///
    /// # Returns
    ///
    /// * `io::Result<()>` - Ok if all items are reported successfully, Err if any fail.
    pub fn report_batch<T: Serialize>(&self, items: &[T]) -> io::Result<()> {
        for item in items {
            self.report_json(item)?;
        }
        Ok(())
    }
}