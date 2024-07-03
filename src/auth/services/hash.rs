use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
/// Hashes a password using bcrypt.
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
/// use sky_rocket_admin::auth::services::auth::hash_password;
/// let hashed = hash_password("my_password".to_string()).unwrap();
/// ```
pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verifies a password against a hashed password.
///
/// # Arguments
///
/// * `password` - The plain text password to verify.
/// * `hash` - The hashed password to verify against.
///
/// # Errors
///
/// This function will return a `BcryptError` if there is an issue with the hashing process
/// or if the password does not match the hashed password. The possible errors include:
/// - `BcryptError::InvalidHash`: Returned when the password does not match the hash.
/// - Other `BcryptError` variants related to the hashing process.
///
pub fn verify_password(password: &str, hash: &str) -> Result<(), BcryptError> {
    match verify(password, hash) {
        Ok(true) => Ok(()),
        Ok(false) => Err(BcryptError::InvalidHash("Invalid password".to_string())),
        Err(e) => Err(e),
    }
}


