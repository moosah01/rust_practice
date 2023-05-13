use std::io;

fn encrypt(input: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let offset = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as u8 - offset + shift) % 26 + offset) as char;
            result.push(shifted);
        } else {
            result.push(c);
        }
    }
    result
}

fn decrypt(input: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let offset = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as u8 - offset + 26 - shift) % 26 + offset) as char;
            result.push(shifted);
        } else {
            result.push(c);
        }
    }
    result
}

fn main() {
    println!("Enter a string to encrypt:");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read input");

    println!("Enter a shift value:");
    let mut shift = String::new();
    io::stdin()
        .read_line(&mut shift)
        .expect("Failed to read input");

    let shift: u8 = shift.trim().parse().expect("Invalid shift value");

    let encrypted = encrypt(&input_string, shift);
    println!("Encrypted string: {:?}", encrypted);

    let decrypted = decrypt(&encrypted, shift);
    println!("Decrypted string: {:?}", decrypted);
}