

// Hash password function
pub fn hash_password(password: String) -> String {
    // Implement your hashing logic here, for example using bcrypt
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}