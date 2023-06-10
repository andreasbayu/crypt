/**
 * Library WASM
 *
 * Library ini digunakan untuk melakukan encoding dan decoding
 * menggunakan algoritma RSA.
 *
 */
use wasm_bindgen::prelude::*;

// recursive
pub fn gcd(a: u32, h: u32) -> u32 {
    // fungsi ini mengembalikan GCD (greatest common divisor)
    if h == 0 {
        a
    } else {
        gcd(h, a % h)
    }
}

#[wasm_bindgen]
pub fn encode(p: u32, q: u32, msg: u32) -> u32 {
    // Stores the first part of public key:
    let n = p * q;

    // Finding the other part of public key.
    // double e stands for encrypt
    let mut e = 2;
    let phi = (p - 1) * (q - 1);

    while e < phi {
        /*
         * e must be co-prime to phi and
         * smaller than phi.
         */
        if gcd(e, phi) == 1 {
            break;
        } else {
            e += 1;
        }
    }

    let mut c: u32 = msg.pow(e.try_into().unwrap());
    c = c % n;

    c
}

#[wasm_bindgen]
pub fn decode(p: u32, q: u32, c: u32) -> u32 {
    // Stores the first part of public key:
    let n = p * q;

    // Finding the other part of public key.
    // double e stands for encrypt
    let mut e = 2;
    let phi = (p - 1) * (q - 1);

    while e < phi {
        /*
         * e must be co-prime to phi and
         * smaller than phi.
         */
        if gcd(e, phi) == 1 {
            break;
        } else {
            e += 1;
        }
    }

    let k = 2; //A constant value

    let d = (1 + (k * phi)) / e;

    // Decryption m = (c ^ d) % n
    let mut m = c.pow(d.try_into().unwrap());
    m = m % n;
    
    m
}

#[wasm_bindgen]
pub fn generate() -> u32 {
    let p = 3;
    let q = 7;

    // Stores the first part of public key:
    let n = p * q;

    // Finding the other part of public key.
    // double e stands for encrypt
    let mut e = 2;
    let phi = (p - 1) * (q - 1);

    while e < phi {
        /*
         * e must be co-prime to phi and
         * smaller than phi.
         */
        if gcd(e, phi) == 1 {
            break;
        } else {
            e += 1;
        }
    }

    let k = 2; //A constant value

    let d = (1 + (k * phi)) / e;

    // Message to be encrypted
    let msg: u32 = 11;

    println!("Message: {} ", msg);

    // Encryption c = (msg ^ e) % n
    let mut c: u32 = msg.pow(e.try_into().unwrap());
    c = c % n;
    println!("Encrypted: {}", c);

    // Decryption m = (c ^ d) % n
    let mut m = c.pow(d.try_into().unwrap());
    m = m % n;
    println!("Original Message: {}", m);

    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        assert_eq!(generate(), 1);
    }
}
