use cryptix_macros::cx;
use std::process::Command;

fn main() {
    let secret_xor = cx!("private-token", "xor");
    println!("Decrypted XOR: {}", secret_xor);

    let secret_aes = cx!("private-token", "aes");
    println!("Decrypted AES: {}", secret_aes);

    let secret_b64 = cx!("private-token", "b64");
    println!("Decode Base64: {}", secret_b64);

    Command::new(cx!("powershell.exe", "xor"))
        .arg(cx!("-NoProfile", "aes"))
        .arg(cx!("-Command", "b64"))
        .arg(cx!("[System.Diagnostics.Process]::Start([System.Text.Encoding]::Unicode.GetString([Convert]::FromBase64String('YwBhAGwAYwA=')))", "aes"))
        .status()
        .expect("failed to execute process");
}

