#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::Request;
use rocket_contrib::Template;


#[derive(Serialize)]
struct Quote {
    id: String,
    text: String
}

#[derive(Serialize)]
struct QuoteListContext {
    items: Vec<Quote>
}

#[derive(Serialize)]
struct SingleQuoteContext {
    item: Quote
}


#[get("/")]
fn index() -> Template {
    let context = QuoteListContext {
        items: vec![
            Quote { id: String::from("0"), text: String::from("rip irc") },
            Quote { id: String::from("1"), text: String::from("take it") },
            Quote { id: String::from("2"), text: String::from("...") },
        ]
    };
    Template::render("index", &context)
}

#[get("/quote/<id>")]
fn quote(id: String) -> Template {
    let context = SingleQuoteContext {
        item: Quote {
            id,
            text: String::from("rip irc\nrip irc\nrip irc")
        }
    };
    Template::render("quote", &context)
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}


fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, quote, files])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
