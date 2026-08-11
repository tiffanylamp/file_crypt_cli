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

    /// REFERENCE EXAMPLE (&[u8]):
    /// Inspects byte slice data without taking ownership.
    /// Demonstrates: Immutable variables, expressions (match), loops.
    pub fn validate_key(&self) -> bool {
        let min_length = 4; // Immutable variable
        let key_len = self.get_key_length();

        // Expression statement evaluating condition
        let is_valid = if key_len >= min_length {
            true
        } else {
            false
        };

        is_valid
    }

    /// OWNERSHIP EXAMPLE (Takes Vec<u8> by value):
    /// Transforms an input byte buffer using XOR cipher logic.
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

// Main function to test how Rust handles ownership moves versus borrowed references

fn main() {
    println!("=== Testing CipherEngine & Ownership Rules ===");

    // 1. Create an owned String key
    let secret_key = String::from("SecretKey123");

    // 2. Pass ownership of `secret_key` into `CipherEngine::new`
    let engine = CipherEngine::new(secret_key, 1);

    // UNCOMMENTING THIS LINE WILL CAUSE A COMPILER ERROR:
    // println!("Key was: {}", secret_key); 
    // Reason: `secret_key` was MOVED into `engine` and is no longer valid in main's scope.

    // 3. Borrow reference (&engine) to validate key
    if engine.validate_key() {
        println!("CipherEngine initialized with key length: {}", engine.get_key_length());
    } else {
        println!("Warning: Key is too short!");
        return;
    }

    // 4. Create sample file byte buffer (Vec<u8>)
    let raw_data = String::from("Hello, Security World!").into_bytes();
    println!("\nOriginal Text Bytes: {:?}", raw_data);

    // 5. OWNERSHIP MOVE: `raw_data` is moved into `process_bytes`
    let encrypted_data = engine.process_bytes(raw_data);

    // UNCOMMENTING THIS LINE WILL CAUSE A COMPILER ERROR:
    // println!("Raw data: {:?}", raw_data); 
    // Reason: `raw_data` was consumed by `process_bytes`.

    println!("Encrypted Bytes:    {:?}", encrypted_data);

    // 6. Decrypting: XOR is symmetric, so processing encrypted_data again restores original
    let decrypted_data = engine.process_bytes(encrypted_data);
    let decrypted_text = String::from_utf8_lossy(&decrypted_data);

    println!("Decrypted Text:     {}", decrypted_text);
}
