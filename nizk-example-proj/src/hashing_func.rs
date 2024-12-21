use sha2::{Sha256, Digest};

pub fn hash_const(b: u32, x: u32, s: u32, p: u32) -> u32 {
    // Convert integers to bytes
    let mut hasher = Sha256::new();
    // Feed each integer as bytes to the hasher
    hasher.update(&b.to_le_bytes()); // Little-endian representation
    hasher.update(&x.to_le_bytes());
    hasher.update(&s.to_le_bytes());
    // Compute the hash
    let result = hasher.finalize();

    // Print the resulting hash in hexadecimal format
    println!("SHA-256 hash: {:x}", result);

    // Get a small number from somewhere in the middle
    let c = u32::from_be_bytes(result.as_slice()[12..16].try_into().unwrap());
    println!("Small number from the middle: {}", c);

    // // 0x8b % 4 == 139 % 4 == 3.
    let r_c = c % p;

    println!("Hash converted to an integer: {}", r_c);

    return r_c;
}
