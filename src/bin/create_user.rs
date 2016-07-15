extern crate iron_test;
extern crate diesel;

use self::iron_test::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Username:");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character
    println!("\nOk! Set password for {} (Press {} when finished)\n", username, EOF);
    let mut passwd = String::new();
    stdin().read_to_string(&mut passwd).unwrap();

    let user = create_user(&connection, username, &passwd);
    println!("\nSaved User {} with password {}", user.username, user.passwd);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
