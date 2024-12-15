pub mod errors;

use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, pkcs1::EncodeRsaPublicKey};
use rand::rngs::OsRng;
use rsa::PaddingScheme;
use std::error::Error;

pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: RsaPrivateKey,
}

/// Generate a 1024-bit RSA key pair.
pub fn generate_keypair() -> Result<KeyPair, Box<dyn Error>> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 1024)?;
    let public_key = RsaPublicKey::from(&private_key);
    let der = public_key_to_der(&public_key)?;
    Ok(KeyPair {
        public_key: der,
        private_key,
    })
}



/// Convert a public key to DER format.
pub fn public_key_to_der(public_key: &RsaPublicKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let der = public_key.to_pkcs1_der()?.as_bytes().to_vec();
    Ok(der)
}

/// Convert DER to PEM format.
pub fn der_to_pem(der: &[u8]) -> String {
    let base64_encoded = base64::encode(der);
    format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", base64_encoded)
}

/// Encrypt data using the public key.
pub fn encrypt_with_public_key(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut rng = OsRng;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key.encrypt(&mut rng, padding, data)?;
    Ok(encrypted_data)
}

/// Decrypt data using the private key.
pub fn decrypt_with_private_key(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_data = private_key.decrypt(padding, encrypted_data)?;
    Ok(decrypted_data)
}

// #-------------- OUTDATED --------------#
//    // Generate RSA key pair
//    let (private_key, public_key) = generate_keypair()?;
//
//    // Convert public key to DER format
//    let der = public_key_to_der(&public_key)?;
//
//    // Convert DER to PEM format and print
//    let pem = der_to_pem(&der);
//    println!("Public Key (PEM):\n{}", pem);
//
//    // Example data to encrypt
//    let data = b"Hello, Minecraft Server!";
//
//    // Encrypt with public key
//    let encrypted_data = encrypt_with_public_key(&public_key, data)?;
//    println!("Encrypted Data: {:?}", encrypted_data);
