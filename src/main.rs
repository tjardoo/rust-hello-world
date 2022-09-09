mod authentication;
use regex::Regex;

fn main() {
    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    println!("The password is: {}", user.get_password());

    let regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    println!("Did our date match? {}", regex.is_match("2014-01-01"));
}