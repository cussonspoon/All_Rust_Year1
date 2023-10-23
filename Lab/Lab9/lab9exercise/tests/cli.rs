use std::process::Command;
use tempfile::NamedTempFile;


#[test]
fn test_integration() {
    let layers = "5";
    let output_file = NamedTempFile::new().expect("Failed to create temporary file");
    let output_file_path = output_file.path().to_str().expect("Failed to get temporary file path");
    let output = Command::new("target/debug/program1")
        .args(&["-l", layers, "-o", output_file_path])
        .output()
        .expect("Failed to run application");
    assert!(output.status.success());

    let file_contents = std::fs::read_to_string(output_file_path).expect("Failed to read output file");
    
    assert!(file_contents.contains("Layer"));
}