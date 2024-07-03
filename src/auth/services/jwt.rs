use serde::{Deserialize, Serialize};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use thiserror::Error;
use jsonwebtoken::{decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::request::{FromRequest, Outcome, Request};

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
/// Generates a JWT token.
///
/// # Arguments
/// * `id` - The user ID.
/// * `username` - The username of the user.
/// * `email` - The email of the user.
/// * `token_duration` - The duration for which the token is valid, in seconds.
///
/// # Returns
/// * `Result<String, TokenError>` - The generated JWT token as a string, or an error if token generation fails.
///
/// # Errors
/// This function will return a `TokenError` if there is an issue with:
/// - Calculating the expiration time.
/// - Encoding the JWT token.
///
/// # Example
/// ```
/// use sky_rocket_admin::auth::services::jwt::get_jwt_token;
///
/// let token = get_jwt_token(1, "username", "email@example.com", 3600).unwrap();
/// ```
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
            eprintln!("System time is before UNIX epoch. Error: {e:?}");
            TokenError::SystemTime(e)
        })?;

    usize::try_from(expiration).map_err(|e| {
        eprintln!("Expiration value is too large for usize. Error: {e:?}");
        TokenError::ConversionError(e)
    })
}
#[derive(Debug)]
pub struct AuthenticatedUser {
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((rocket::http::Status::Unauthorized, ()));
        }

        let token = keys[0].replace("Bearer ", "");
        let key = DecodingKey::from_secret("your_secret_key".as_ref());
        let validation = Validation::default();

        match decode::<Claims>(&token, &key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;
                Outcome::Success(AuthenticatedUser {
                    username: claims.sub,
                })
            }
            Err(_) => Outcome::Error((rocket::http::Status::Unauthorized, ())),
        }
    }
}