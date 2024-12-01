use std::fs;
use std::path::Path;

/// Reads the appropriate input file (example or real input) for the module calling it.
///
/// # Arguments
/// * `module_path` - The `file!()` macro from the calling module to determine the file location.
/// * `example` - If true, reads the `example.txt`; otherwise, reads `input.txt`.
///
/// # Returns
/// The content of the file as a String.
pub fn read_input(module_path: &str, example: bool) -> String {
    let dir = Path::new(module_path).parent().expect("Failed to determine module directory");
    let file_name = if example { "example.txt" } else { "input.txt" };
    let file_path = dir.join(file_name);

    fs::read_to_string(file_path).expect("Failed to read input file")
}
