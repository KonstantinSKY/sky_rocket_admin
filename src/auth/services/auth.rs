use bcrypt::{hash ,BcryptError, DEFAULT_COST};


/// Hashes a password using bcrypt.
///
/// # Arguments
///
/// * `password` - The plain text password to hash.
///
/// # Returns
///
/// * `Result<String, BcryptError>` - The hashed password, or an error if hashing fails.
///
/// # Errors
///
/// This function will return a `BcryptError` if the password hashing fails.
///
/// # Example
/// ```
/// let hashed = hash_password("my_password".to_string()).unwrap();
/// ```
pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

// pub fn authorize_user(user: &User, credentials: Credentials) -> Result<String, Error> {
//     let argon2 = Argon2::default();
//     let db_hash = PasswordHash::new(&user.password)?;
//     argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;

//     let session_id = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(128)
//         .map(char::from)
//         .collect();

//     Ok(session_id)
// }
