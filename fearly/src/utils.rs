use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    parsed_hash.to_string()
}

pub fn password_verifier(password: &[u8], hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    let status = Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok();
    status
}


pub fn errorln(msg: impl std::fmt::Display) {
    eprintln!("\x1b[31m❌ {}\x1b[0m", msg);
}
