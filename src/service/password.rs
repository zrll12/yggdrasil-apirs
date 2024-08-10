use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use pbkdf2::pbkdf2_hmac_array;
use rand::Rng;
use sha2::Sha256;

const ITERATIONS: u32 = 100_000;
const KEY_LEN: usize = 32;

pub fn generate_password_hash(password: &str) -> String {
    // 生成随机盐
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789-";
    const SALT_LEN: usize = 16;

    let mut rng = rand::thread_rng();

    let salt: String = (0..SALT_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let hash = pbkdf2_hmac_array::<Sha256, KEY_LEN>(password.as_bytes(), salt.as_bytes(), ITERATIONS);
    let hash = BASE64_STANDARD.encode(hash);

    format!("{}${}", salt, hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parts: Vec<&str> = password_hash.split('$').collect();
    if parts.len() != 2 {
        return false;
    }

    let salt = parts[0];
    let hash = parts[1];

    let hash_calc = pbkdf2_hmac_array::<Sha256, KEY_LEN>(password.as_bytes(), salt.as_bytes(), ITERATIONS).to_vec();
    let hash_calc = BASE64_STANDARD.encode(&hash_calc);

    hash_calc == hash
}

#[test]
fn test_hash_password() {
    let password = "Test123";
    let hash = generate_password_hash(password);

    println!("Password hash: {}", &hash);

    assert!(verify_password(password, &hash))
}