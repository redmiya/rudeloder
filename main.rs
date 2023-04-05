extern crate hound;

use hound::WavReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path/to/your/wav/unko.wav>", args[0]);
        return;
    }

    let filepath = &args[1];
    let encrypted_code =
        extract_encrypted_code(filepath).expect("Failed to extract encrypted code"); // Extract Encrypted Codes

    let decrypted_code = decrypt_code(&encrypted_code);
    execute_code(&decrypted_code);
}

fn extract_encrypted_code(filepath: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut buffer = [0; 44]; // Read first 44 bytes of WAV header
    file.read_exact(&mut buffer)?;

    let encrypted_code_start = 36; // Assuming the encrypted code is at position 36 in header
    let encrypted_code_length = 4; // Assuming the length of the encrypted code is 4 bytes
    let encrypted_code =
        buffer[encrypted_code_start..encrypted_code_start + encrypted_code_length].to_vec();

    Ok(encrypted_code)
}

fn decrypt_code(encrypted_code: &[u8]) -> Vec<u8> {
    // Replace this with your decryption algorithm
    encrypted_code.to_vec()
}

fn execute_code(decrypted_code: &[u8]) {
    // Replace this with your code execution logic
    println!("Decrypted code: {:?}", decrypted_code);
}
