extern crate iron_test;
extern crate diesel;

use self::iron_test::*;
use self::iron_test::models::*;
use self::diesel::prelude::*;

fn main() {
    use iron_test::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------\n");
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
