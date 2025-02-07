use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

/// Represents different types of reporters for output handling
pub enum ReporterType {
    /// File-based reporter that writes to a specified file
    File(FileReporter),
    /// Console-based reporter that writes to stdout
    Console(ConsoleReporter),
}

impl ReporterType {
    /// Reports formatted data using the Display trait
    ///
    /// # Arguments
    /// * `data` - The data to be reported implementing Display trait
    pub fn report<T: std::fmt::Display>(&self, data: &T) {
        match self {
            ReporterType::File(r) => r.report(data),
            ReporterType::Console(r) => r.report(data),
        }
    }

    /// Reports data in JSON format
    ///
    /// # Arguments
    /// * `data` - The data to be serialized and reported
    pub fn report_json<T: Serialize>(&self, data: &T) {
        match self {
            ReporterType::File(r) => r.report_json(data),
            ReporterType::Console(r) => r.report_json(data),
        }
    }

    /// Reports a batch of items in JSON format
    ///
    /// # Arguments
    /// * `items` - Slice of items to be reported
    pub fn report_batch<T: Serialize>(&self, items: &[T]) {
        match self {
            ReporterType::File(r) => r.report_batch(items),
            ReporterType::Console(r) => r.report_batch(items),
        }
    }
}

/// File-based reporter that writes output to a file or stdout
pub struct FileReporter {
    /// Thread-safe writer that can be either a file or stdout
    writer: Mutex<Box<dyn Write + Send>>,
}

/// Console-based reporter that writes output to stdout
pub struct ConsoleReporter;

impl FileReporter {
    /// Creates a new FileReporter
    ///
    /// # Arguments
    /// * `file_path` - Optional path to output file. If None, uses stdout
    ///
    /// # Returns
    /// A new FileReporter instance
    pub fn new(file_path: Option<&str>) -> Self {
        let writer = match file_path {
            Some(path) => {
                let file = File::create(Path::new(path)).unwrap();
                Box::new(file) as Box<dyn Write + Send>
            }
            None => Box::new(io::stdout()) as Box<dyn Write + Send>,
        };
        Self {
            writer: Mutex::new(writer),
        }
    }

    /// Reports formatted data with newline
    ///
    /// # Arguments
    /// * `data` - The data to be reported
    pub fn report<T: std::fmt::Display>(&self, data: &T) {
        let output = format!("{}\n", data);
        self.writer
            .lock()
            .unwrap()
            .write_all(output.as_bytes())
            .unwrap();
    }

    /// Reports data in JSON format
    ///
    /// # Arguments
    /// * `data` - The data to be serialized and reported
    pub fn report_json<T: Serialize>(&self, data: &T) {
        let json = serde_json::to_string(data).unwrap();
        self.report(&json);
    }

    /// Reports multiple items in JSON format
    ///
    /// # Arguments
    /// * `items` - Slice of items to be reported
    pub fn report_batch<T: Serialize>(&self, items: &[T]) {
        for item in items {
            self.report_json(item);
        }
    }
}

impl ConsoleReporter {
    /// Creates a new ConsoleReporter
    pub fn new() -> Self {
        Self
    }

    /// Reports formatted data to console
    ///
    /// # Arguments
    /// * `data` - The data to be reported
    pub fn report<T: std::fmt::Display>(&self, data: &T) {
        println!("{}", data);
    }

    /// Reports data in JSON format to console
    ///
    /// # Arguments
    /// * `data` - The data to be serialized and reported
    pub fn report_json<T: Serialize>(&self, data: &T) {
        println!("{}", serde_json::to_string(data).unwrap());
    }

    /// Reports multiple items in JSON format to console
    ///
    /// # Arguments
    /// * `items` - Slice of items to be reported
    pub fn report_batch<T: Serialize>(&self, items: &[T]) {
        for item in items {
            self.report_json(item);
        }
    }
}
