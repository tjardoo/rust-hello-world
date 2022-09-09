mod authentication {
    pub struct User {
        username: String,
        password_hash: String,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: password.to_string(),
            }
        }

        pub fn get_username(&self) -> &String {
            return &self.username;
        }
    
        pub fn get_password(&self) -> &String {
            return &self.password_hash;
        }
    }
}

fn main() {

    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    println!("The password is: {}", user.get_password());

}