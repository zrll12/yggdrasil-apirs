use std::fs;
use std::io::Read;
use lazy_static::lazy_static;
use rand::thread_rng;
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey, DecodeRsaPublicKey};
use rsa::pkcs8::LineEnding;
use rsa::{RsaPrivateKey, RsaPublicKey};
use log::debug;

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
    pub_key
        .write_pkcs1_pem_file("keys/public.pem", LineEnding::LF)
        .unwrap();

    (priv_key, pub_key)
}

pub fn get_key_pair() -> (RsaPrivateKey, RsaPublicKey) {
    if fs::metadata("keys/private.pem").is_ok() && fs::metadata("keys/public.pem").is_ok() {
        let mut private_key = String::new();
        let mut public_key = String::new();

        fs::File::open("keys/private.pem")
            .unwrap()
            .read_to_string(&mut private_key)
            .unwrap();
        fs::File::open("keys/public.pem")
            .unwrap()
            .read_to_string(&mut public_key)
            .unwrap();

        let private_key = RsaPrivateKey::from_pkcs1_pem(&private_key).unwrap();
        let public_key = RsaPublicKey::from_pkcs1_pem(&public_key).unwrap();
        (private_key, public_key)
    } else {
        generate_key_pair()
    }
}
