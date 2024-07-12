use std::fs;
use std::io::Read;

use base64::Engine;
use lazy_static::lazy_static;
use log::debug;
use rand::thread_rng;
use rsa::pkcs1::{
    DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey,
};
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::{Hash, PaddingScheme, RsaPrivateKey, RsaPublicKey};
use sha1::{Digest, Sha1};

lazy_static! {
    pub static ref SIGNATURE_KEY_PAIR: (RsaPrivateKey, RsaPublicKey) = get_key_pair();
}

fn generate_key_pair() -> (RsaPrivateKey, RsaPublicKey) {
    debug!("Generating new key pair.");
    fs::create_dir_all("keys").unwrap();
    let mut rng = thread_rng();
    let bits = 4096;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    debug!("New key pair generated.");
    priv_key
        .write_pkcs1_pem_file("keys/private.pem", LineEnding::LF)
        .unwrap();

    (priv_key, pub_key)
}

pub fn get_key_pair() -> (RsaPrivateKey, RsaPublicKey) {
    if fs::metadata("keys/private.pem").is_ok() {
        let mut private_key = String::new();

        fs::File::open("keys/private.pem")
            .unwrap()
            .read_to_string(&mut private_key)
            .unwrap();

        let private_key = RsaPrivateKey::from_pkcs1_pem(&private_key).unwrap();
        let public_key = RsaPublicKey::from(&private_key);
        (private_key, public_key)
    } else {
        generate_key_pair()
    }
}

pub fn rsa_sign(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let hashed_data = hasher.finalize();
    let hash = hashed_data.as_slice();

    let signature = SIGNATURE_KEY_PAIR.0.sign(PaddingScheme::PKCS1v15Sign {hash: Option::from(Hash::SHA1) }, &hash).unwrap();
    base64::engine::general_purpose::STANDARD.encode(&signature)
}
