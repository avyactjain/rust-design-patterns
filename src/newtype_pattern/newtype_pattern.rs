use std::{fmt::Display, ops::Deref};

pub struct UserName(String);

impl Deref for UserName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let name = "Avyact Jain";
    let username = UserName(name.to_string());

    let deref = username.deref();
    println!("Deref is {}", deref);
    println!("Username is {}", username);
}
