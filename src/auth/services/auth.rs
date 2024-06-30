

// Hash password function
pub fn hash_password(password: String) -> String {
    // Implement your hashing logic here, for example using bcrypt
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
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
