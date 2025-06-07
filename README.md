<p align="center"> <img src="https://github.com/user-attachments/assets/0e25be3e-0e7c-484c-b465-3ee7e0a9ab45" alt="Your image" /> </p>

# Cryptix: Compile-Time String Encryption & Obfuscation Framework for Rust

Cryptix is a Rust-based compile-time string encryption framework designed to protect sensitive literals from static analysis tools. It provides a powerful set of macro-based tools that obfuscate string constants during compilation, decrypting them only at runtime in memory-safe and optionally customizable ways.

✨ Inspired by [litcrypt](https://github.com/anvie/litcrypt.rs)
- This project is heavily inspired by `litcrypt`, a compile-time literal encryption crate.

🔧 Why Cryptix?
Unlike other crates like litcrypt, Cryptix offers:
- More encryption options.
- Fully customizable encoding behavior.

## 🚀 Key Features

- Compile-Time Encryption: Encrypts string literals at compile time using a variety of algorithms.

- Macro-Based Usage: Simple two-letter macro interface for embedding encrypted strings in your code (e.g., `cx!("my string", "<method>")`).

- Multiple Encryption Modules:
	- `aes`: AES-256-CBC with PKCS7 padding, no block-modes crate.
	- `xor`: Lightweight XOR-based obfuscation.
	- `base64`: Custom base64 encoding with a shuffled alphabet.

- Static Key/IV: Secret keys and IVs are embedded directly in the library, no need for environment variables.

- Cross-Platform: Works on both Linux and Windows.

## 📦 Example Usage

```yaml
# Cargo.toml

[dependencies]
cryptix_1988 = "0.6.0"
cryptix_macros = "0.3.0"
```

```rust
//main.rs

use cryptix_macros::cx; // new
use std::process::Command;

fn main() {
    let secret_xor = cx!("private-token", "xor");
    println!("Decrypted XOR: {}", secret_xor);

    let secret_aes = cx!("private-token", "aes");
    println!("Decrypted AES: {}", secret_aes);

    let secret_b64 = cx!("private-token", "b64");
    println!("Decode Base64: {}", secret_b64);

    Command::new(cx!("powershell.exe", "xor")) // "xor" method
        .arg(cx!("-NoProfile", "aes")) // "aes" method
        .arg(cx!("-Command", "b64")) // "b64" method
        .arg(cx!("[System.Diagnostics.Process]::Start([System.Text.Encoding]::Unicode.GetString([Convert]::FromBase64String('YwBhAGwAYwA=')))", "aes")) // "aes" method
        .status()
        .expect("failed to execute process");
}
```

This will:

- Encrypt "private-token" at compile time using the xor encryption module.
- Store it encrypted in your binary (not visible via static tools: `strings`, `hexdump`, etc).
- Decrypt it at runtime just-in-time for use.

Cryptix will encrypt each string written inside cx! statically.

Check the output binary using strings command to verify:

```bash
strings target/debug/my_app | grep private-token
```

If the output is blank then your valuable string in your app is safe from static analyzer tool

For working example code see `./examples` directory, and test using:

```bash
cargo run
```

## Disclaimer

All the code are for educational and research purposes only. Do not attempt to violate the law with anything contained in code produced by this repository. Neither the authors of this repository, or anyone else affiliated in any way, is going to accept responsibility for your actions.