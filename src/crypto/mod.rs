use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use base58::{FromBase58, ToBase58};
use base64::{Engine as _, engine::general_purpose};
use std::error::Error;

pub fn generate_keypair() -> (String, String) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    let secret_bytes = signing_key.to_bytes();
    let public_bytes = verifying_key.to_bytes();
    
    let secret = secret_bytes.to_vec().to_base58();
    let pubkey = public_bytes.to_vec().to_base58();
    
    (pubkey, secret)
}

pub fn sign_message(message: &str, secret_key_b58: &str) -> Result<(String, String), Box<dyn Error>> {
    let secret_bytes = secret_key_b58.from_base58().map_err(|e| format!("Base58 decode error: {:?}", e))?;
    if secret_bytes.len() != 32 {
        return Err("Invalid secret key length".into());
    }
    
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&secret_bytes);
    let signing_key = SigningKey::from_bytes(&key_bytes);
    
    let verifying_key = signing_key.verifying_key();
    let pubkey = verifying_key.to_bytes().to_vec().to_base58();
    
    let signature = signing_key.sign(message.as_bytes());
    
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    
    Ok((signature_b64, pubkey))
}

pub fn verify_signature(message: &str, signature_b64: &str, pubkey_b58: &str) -> Result<bool, Box<dyn Error>> {
    let pubkey_bytes = pubkey_b58.from_base58().map_err(|e| format!("Base58 decode error: {:?}", e))?;
    if pubkey_bytes.len() != 32 {
        return Err("Invalid public key length".into());
    }
    
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&pubkey_bytes);
    let verifying_key = VerifyingKey::from_bytes(&key_bytes)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    
    let signature_bytes = general_purpose::STANDARD.decode(signature_b64)
        .map_err(|e| format!("Base64 decode error: {:?}", e))?;
    if signature_bytes.len() != 64 {
        return Err("Invalid signature length".into());
    }
    
    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(&signature_bytes);
    let signature = Signature::from_bytes(&sig_bytes);
    
    match verifying_key.verify(message.as_bytes(), &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
} 