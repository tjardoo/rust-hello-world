pub struct User {
    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(&password),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password(&self) -> &String {
        &self.password_hash
    }
}

fn hash_password(plain_password: &str) -> String {
    return String::from(plain_password);
}