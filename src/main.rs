use std::fs;
use std::io::{self, Write};
use std::path::Path;

// ==========================================
// 1. STRUCT & IMPL BLOCK (Requirement: OOP)
// ==========================================

/// Encapsulates cryptographic configuration and key material.
pub struct CipherEngine {
    key: String,
    rounds: u8,
}

impl CipherEngine {
    /// Constructor method: Constructs a new CipherEngine.
    /// Takes ownership of `key_input` (String).
    pub fn new(key_input: String, rounds: u8) -> Self {
        CipherEngine {
            key: key_input,
            rounds,
        }
    }

    /// REFERENCE EXAMPLE (&self):
    /// Reads internal state without taking ownership or mutating the struct.
    pub fn get_key_length(&self) -> usize {
        self.key.len()
    }

    /// REFERENCE & SLICING EXAMPLE:
    /// Inspects key validity using a byte slice.
    pub fn validate_key(&self) -> bool {
        let key_bytes = self.key.as_bytes();
        let min_length = 4;

        if key_bytes.len() < min_length {
            return false;
        }

        // Requirement: Slicing (inspects prefix slice)
        let prefix_slice = &key_bytes[0..4];
        !prefix_slice.is_empty()
    }

    /// OWNERSHIP EXAMPLE (Takes Vec<u8> by value):
    /// Consumes `data` and returns a newly mutated owned `Vec<u8>`.
    pub fn process_bytes(&self, mut data: Vec<u8>) -> Vec<u8> {
        let key_bytes = self.key.as_bytes();
        let key_len = key_bytes.len();

        if key_len == 0 {
            return data;
        }

        // Loop over byte buffer, applying XOR transformation per round
        for round in 0..self.rounds {
            for (i, byte) in data.iter_mut().enumerate() {
                let key_byte = key_bytes[(i + round as usize) % key_len];
                *byte ^= key_byte; // Bitwise XOR
            }
        }

        data // Return ownership of transformed vector
    }
}

// ==========================================
// 2. FILE I/O HELPERS (Requirement: Functions)
// ==========================================

/// Reads raw byte buffer from a given file path.
pub fn read_file_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    let path = Path::new(file_path);
    fs::read(path)
}

/// Writes processed byte buffer back to a file.
pub fn write_file_bytes(file_path: &str, data: &[u8]) -> io::Result<()> {
    let path = Path::new(file_path);
    fs::write(path, data)
}

// Helper to handle interactive terminal prompts
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// ==========================================
// 3. MAIN INTERACTIVE CLI
// ==========================================

fn main() {
    println!("============================================");
    println!("     CLI FILE ENCRYPTION & DECRYPTION      ");
    println!("============================================");

    loop {
        println!("\nSelect an action:");
        println!("1. Encrypt / Decrypt a File");
        println!("2. Run Verification Test");
        println!("3. Exit");

        let choice = get_user_input("\nEnter choice (1, 2, or 3): ");

        match choice.as_str() {
            "1" => {
                let file_path = get_user_input("Enter target file path (e.g. sample.txt): ");

                if !Path::new(&file_path).exists() {
                    println!("[!] Error: File '{}' does not exist!", file_path);
                    continue;
                }

                let secret_key = get_user_input("Enter encryption key (min 4 chars): ");
                let engine = CipherEngine::new(secret_key, 1);

                if !engine.validate_key() {
                    println!("[!] Error: Key length must be at least 4 characters!");
                    continue;
                }

                let output_path = get_user_input("Enter output file path (e.g. encrypted.bin): ");

                match read_file_bytes(&file_path) {
                    Ok(raw_bytes) => {
                        println!("[+] Processing {} bytes...", raw_bytes.len());
                        let transformed_bytes = engine.process_bytes(raw_bytes);

                        if let Err(e) = write_file_bytes(&output_path, &transformed_bytes) {
                            println!("[!] Write Error: {}", e);
                        } else {
                            println!("[✓] Success! Processed file saved to '{}'", output_path);
                        }
                    }
                    Err(e) => println!("[!] Read Error: {}", e),
                }
            }
            "2" => {
                println!("\n--- Running Internal Test ---");
                let test_key = String::from("SecretKey123");
                let test_engine = CipherEngine::new(test_key, 1);
                let sample_data = String::from("Hello, Security World!").into_bytes();

                println!("Original Bytes:  {:?}", sample_data);
                let encrypted = test_engine.process_bytes(sample_data);
                println!("Encrypted Bytes: {:?}", encrypted);
                let decrypted = test_engine.process_bytes(encrypted);
                println!("Restored Text:   '{}'", String::from_utf8_lossy(&decrypted));
            }
            "3" => {
                println!("Exiting application. Goodbye!");
                break;
            }
            _ => println!("[!] Invalid choice. Please pick 1, 2, or 3."),
        }
    }
}