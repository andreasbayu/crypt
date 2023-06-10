use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn vigenere_encode(plaintext: &str, key: &str) -> String {
    let plaintext = plaintext.to_uppercase();
    let key = key.to_uppercase();
    let key_length = key.len();
    let mut ciphertext = String::new();

    for (i, c) in plaintext.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            let key_char = key.chars().nth(i % key_length).unwrap();
            let key_shift = key_char as u8 - b'A';
            let encrypted_char = ((((c as u8 - b'A') + key_shift) % 26) + b'A') as char;
            ciphertext.push(encrypted_char);
        } else {
            ciphertext.push(c);
        }
    }

    ciphertext
}

#[wasm_bindgen]
pub fn vigenere_decode(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();
    let key = key.to_uppercase();
    let ciphertext = ciphertext.to_uppercase();

    for (i, ciphertext_char) in ciphertext.chars().enumerate() {
        if ciphertext_char.is_ascii_alphabetic() {
            let key_char = key.chars().nth(i % key.len()).unwrap();
            let key_shift = (key_char as u8 - b'A') as i32;
            let decrypted_char = ((((ciphertext_char as u8 - b'A') as i32 - key_shift + 26) % 26) + b'A' as i32) as u8 as char;
            plaintext.push(decrypted_char);
        } else {
            plaintext.push(ciphertext_char);
        }
    }

    plaintext
}

// fn main() {
//     let plaintext = "Hello, Vigenere!";
//     let key = "KEY";
    
//     let ciphertext = vigenere_encode(plaintext, key);
//     println!("Ciphertext: {}", ciphertext);

//     let decrypted_plaintext = vigenere_decode(&ciphertext, key);
//     println!("Decrypted Plaintext: {}", decrypted_plaintext);
// }


// mod tests {
//     use super::*;

//     #[test]
//     fn mains() {
//         main()
//     }
// }