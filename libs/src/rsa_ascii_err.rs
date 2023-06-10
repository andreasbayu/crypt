use rand::Rng;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn extended_gcd(a: u128, b: u128) -> (u128, u128, u128) {
    if b == 0 {
        return (1, 0, a);
    }
    let (x, y, xgcd) = extended_gcd(b, a % b);

    let x_new = y;
    let y_new = x.wrapping_sub((a / b).wrapping_mul(y));

    (x_new, y_new, xgcd)
}

fn mod_inverse(a: u128, m: u128) -> u128 {
    if gcd(a, m) != 1 {
        panic!("Modular inverse does not exist.");
    }
    let (x, _, _) = extended_gcd(a, m);

    x % m
}

fn generate_keypair() -> ((u128, u128), (u128, u128)) {
    let mut rng = rand::thread_rng();

    // Step 1: Choose two distinct prime numbers
    let p = 61;
    let q = 53;

    // Step 2: Compute n = p * q
    let n = p * q;

    // Step 3: Compute φ(n) = (p - 1) * (q - 1)
    let phi = (p - 1) * (q - 1);

    // Step 4: Choose e such that 1 < e < φ(n) and gcd(e, φ(n)) = 1
    let mut e = rng.gen_range(1..=phi);

    // Ensure e and φ(n) are coprime
    while gcd(e, phi) != 1 {
        e = rng.gen_range(1..=phi)
    }

    // Step 5: Compute the modular multiplicative inverse of e modulo φ(n)
    let d = mod_inverse(e, phi);

    ((e, n), (d, n))
}

fn encrypt(public_key: (u128, u128), message: &str) -> Vec<u128> {
    let (e, n) = public_key;
    let encrypted_message: Vec<u128> = message
        .chars()
        .map(|c| c as u128)
        .map(|c| c.pow(e as u32) % n)
        .collect();
    encrypted_message
}

fn decrypt(private_key: (u128, u128)    , encrypted_message: &[u128]) -> String {
    let (d, n) = private_key;
    let decrypted_message: String = encrypted_message
        .iter()
        .map(|&char| (char.pow(d as u32) % n) as u8)
        .map(|byte| byte as char)
        .collect();
    decrypted_message
}

fn main() {
    let (public_key, private_key) = generate_keypair();

    // Encryption
    let message = "Hello, RSA!";
    let encrypted_message = encrypt(public_key, message);
    println!("Encrypted message: {:?}", encrypted_message);

    // Decryption
    let decrypted_message = decrypt(private_key, &encrypted_message);
    println!("Decrypted message: {:?}", decrypted_message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emain() {
        main();
    }
}
