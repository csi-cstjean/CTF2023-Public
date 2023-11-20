use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn chiffrement_aes(payload: &str) -> Vec<u8> {
    let pass = "gNGMRqqiqbu4bxOkuTV9pHdB"; // Changer la clé car retrouvable dans le .wasm
    let encrypted = crypter::encrypt(pass.as_bytes(), payload.as_bytes()).expect("vQMLSvy9WUjpmFDj76tu7DYEJqH");

    encrypted
}

#[wasm_bindgen]
pub fn dechiffrement_aes(encrypted: &[u8], key: &str) -> String {
    let decrypted = crypter::decrypt(key.as_bytes(), &encrypted).expect("QC2oJ8tU8KLq3z72HgkK4hhEzQG");
    
    String::from_utf8(decrypted).expect("9iJuYGgrDJUkYk0YV3pKjHnf180")
}
