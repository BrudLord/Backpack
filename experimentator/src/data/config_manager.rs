use crate::models::experiment_config::ExperimentConfig;
use std::fs;
use std::path::Path;

/// Reads and deserializes a JSON configuration file containing experiment parameters.
///
/// This function attempts to read the file specified by `path`, parse it as JSON,
/// and deserialize it into a vector of `ExperimentConfig` structs.  Each element
/// in the vector represents a separate set of experiment parameters. This allows
/// for running multiple experiments with varying parameters from a single config file.
///
/// # Arguments
///
/// * `path`: A path to the JSON configuration file.  This can be any type that
///   implements `AsRef<Path>`, such as `&str` or `String`.
///
/// # Returns
///
/// * `Result<Vec<ExperimentConfig>, String>`:  If successful, returns a `Vec` of `ExperimentConfig` structs, where each struct
///   represents a set of experiment parameters defined in the JSON file.
///   If an error occurs (either during file reading or JSON parsing), returns
///   an `Err` containing a string describing the error.
///
/// # Errors
///
/// This function can return an error if:
/// * The file at the given path cannot be read.
/// * The file content is not valid JSON.
/// * The JSON structure does not match the expected structure for
///   a vector of `ExperimentConfig` structs.
pub fn read_rand_config<P: AsRef<Path>>(path: P) -> Result<Vec<ExperimentConfig>, String> {
    // Reads the file contents into a String.  If the file cannot be read,
    // returns an error with a stringified version of the underlying I/O error.
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    // Parses the JSON string into a vector of `ExperimentConfig` structs.
    // If the JSON is invalid or doesn't match the expected structure,
    // returns an error with a stringified version of the Serde JSON error.
    serde_json::from_str(&data).map_err(|e| e.to_string())
}
