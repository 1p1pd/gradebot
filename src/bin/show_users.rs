extern crate iron_test;
extern crate diesel;

use self::iron_test::*;
use self::iron_test::models::*;
use self::diesel::prelude::*;

fn main() {
    use iron_test::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}: {}", user.username, user.passwd);
    }
}
