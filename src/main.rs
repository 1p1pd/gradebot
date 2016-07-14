extern crate iron;
extern crate router;
extern crate env_logger;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate iron_test;
extern crate diesel;
#[macro_use]
extern crate maplit;

use std::error::Error;

use iron::prelude::*;
use iron::{status};
use router::Router;
use std::io::Read;
use rustc_serialize::json;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use self::iron_test::*;
use self::iron_test::models::*;
use self::diesel::prelude::*;

mod data {
    extern crate iron_test;
    extern crate diesel;

    use self::iron_test::*;
    use self::iron_test::models::*;
    use self::diesel::prelude::*;

    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;
    use iron_test::schema::posts::dsl::*;

    pub struct Team {
        name: String,
        pts: u16
    }

    pub struct Note {
        title: String,
        body: String
    }

    impl ToJson for Team {
        fn to_json(&self) -> Json {
            let mut m: BTreeMap<String, Json> = BTreeMap::new();
            m.insert("name".to_string(), self.name.to_json());
            m.insert("pts".to_string(), self.pts.to_json());
            m.to_json()
        }
    }

    impl ToJson for Note {
        fn to_json(&self) -> Json {
            let mut m: BTreeMap<String, Json> = BTreeMap::new();
            m.insert("title".to_string(), self.title.to_json());
            m.insert("body".to_string(), self.body.to_json());
            m.to_json()
        }
    }

    pub fn make_data () -> BTreeMap<String, Json> {
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), "2015".to_json());

        let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
                                 pts: 431u16 },
                          Team { name: "Beijing Guoan".to_string(),
                                 pts: 27u16 },
                          Team { name: "Guangzhou Evergrand".to_string(),
                                 pts: 22u16 },
                          Team { name: "Shandong Luneng".to_string(),
                                 pts: 12u16 } ];
        let connection = establish_connection();
        let results = posts.filter(published.eq(true))
                    .limit(5)
                    .load::<Post>(&connection)
                    .expect("Error loause iron::Handler;ding posts");
        let mut notes = vec![];
        for post in results {
            notes.push(Note{title: post.title.to_string(), body: post.body.to_string()});
        }

        data.insert("teams".to_string(), teams.to_json());
        data.insert("notes".to_string(), notes.to_json());
        data.insert("engine".to_string(), "rustc_serialize".to_json());
        data
    }
}

/// the handlers
fn index(_: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::new("hello", data)).set_mut(status::Ok);
    Ok(resp)
}

fn input(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("cancer", ())).set_mut(status::Ok);
    Ok(resp)
}

fn login(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("memory", ())).set_mut(status::Ok);
    Ok(resp)
}

fn memory(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("memory", ())).set_mut(status::Ok);
    Ok(resp)
}

fn temp(_: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::with("<h1>{{engine}}</h1>", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    let mem_templates = btreemap! {
        "memory".to_owned() => "<h1>Memory Template</h1>".to_owned()
    };
    // add a memory based source
    hbse.add(Box::new(MemorySource(mem_templates)));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }


    let mut router = Router::new();
    router
        .get("/", index)
        .get("/input", input)
        .get("/login", login)
        .get("/mem", memory)
        .get("/temp", temp);
    let mut chain = Chain::new(router);
    chain.link_after(hbse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
