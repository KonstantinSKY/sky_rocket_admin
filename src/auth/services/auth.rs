use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use thiserror::Error;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm, errors::Error as JwtError};
// use crate::schema::users::{email, username};

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

#[derive(Error, Debug)]


pub enum TokenError {
    #[error("System time error: {0}")]
    SystemTime(#[from] SystemTimeError),
    #[error("Expiration value is too large for usize: {0}")]
    ConversionError(#[from] std::num::TryFromIntError),
    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),
}
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    username: String,
    email: String,
}
pub fn get_jwt_token(id: i32, username: &str, email: &str, token_duration: u64) -> Result<String, TokenError> {
    let exp = calculate_expiration(token_duration)?;

    let claims = Claims {
        sub: id.to_string(),
        username: username.to_owned(),
        email: email.to_owned(),
        exp,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),
    )?;


    Ok(token)
}

fn calculate_expiration(token_duration: u64) -> Result<usize, TokenError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() + token_duration)
        .map_err(|e| {
            eprintln!("System time is before UNIX epoch. Error: {:?}", e);
            TokenError::SystemTime(e)
        })?;

    usize::try_from(expiration).map_err(|e| {
        eprintln!("Expiration value is too large for usize. Error: {:?}", e);
        TokenError::ConversionError(e)
    })
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
