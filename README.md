# Overview

As a software engineer, my goal with this project was to gain hands-on experience with systems programming in Rust. Specifically, I set out to explore low-level memory management, variable ownership rules, borrowing mechanics, and fast bitwise data transformations without relying on external garbage collection or risk of memory leaks.

I developed a command-line **File Encryption and Decryption Utility** in Rust. The application allows users to securely encrypt target files using a secret key and a bitwise XOR transformation engine, save the encrypted payload as binary output, and decrypt the files back to their original state using identical key material.

The primary purpose of writing this software was to deepen my understanding of Rust's unique safety guarantees—such as ownership transfer, mutable vs. immutable borrowing, and pattern matching—while implementing practical File I/O operations and low-level byte buffer manipulation.

Encryption key for testing: MySecretPass123


[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

* **Development Tools:** Visual Studio Code, Git, Cargo (Rust build tool & package manager)
* **Plugins/Extensions:** `rust-analyzer` for inline diagnostics and syntax checking
* **Programming Language:** Rust (Edition 2021)
* **Libraries/Standard Modules:** 
  * `std::fs` (File system operations)
  * `std::io` (Standard input/output and buffer flushing)
  * `std::path::Path` (Path checking and validation)

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example - Ownership and Moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust Standard Library Documentation (`std::fs`)](https://doc.rust-lang.org/std/fs/)

# Future Work

- Implement industry-standard cryptographic algorithms (such as AES-256-GCM via external crates).
- Add support for hashing (e.g., SHA-256 checksums) to verify file integrity before and after decryption.
- Introduce dynamic key derivation functions (PBKDF2/Argon2) instead of raw password byte stretching.