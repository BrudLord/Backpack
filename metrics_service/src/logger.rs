use serde::Serialize;
use serde_json::to_string;
use std::fs::File;
use std::io::{self, Write};

pub struct Logger {
    pub log_to_file: bool,
    pub file: Option<File>,
}

impl Logger {
    pub fn new(log_to_file: bool, file_path: Option<&str>) -> io::Result<Self> {
        let file = if log_to_file {
            Some(File::create(file_path.unwrap())?)
        } else {
            None
        };
        Ok(Logger { log_to_file, file })
    }

    pub fn log_display<T: std::fmt::Display>(&mut self, experiment_unit: &T) {
        let log_entry = &experiment_unit;
        if self.log_to_file {
            if !self.file.is_none() {
                writeln!(self.file.as_mut().unwrap(), "{}\n", log_entry).unwrap();
            }
        } else {
            println!("{}", log_entry);
        }
    }

    pub fn log_serial<T: Serialize>(&mut self, experiment_unit: &T) {
        self.log_display(&to_string(&experiment_unit).unwrap());
    }

    pub fn log_batch_sequential<T: Serialize>(&mut self, experiment_units: &[T]) {
        for unit in experiment_units {
            self.log_serial(unit);
        }
    }
}
