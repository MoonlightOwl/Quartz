#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod db;
mod quote;

use quote::Quote;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::Request;
use rocket_contrib::Template;

#[derive(Serialize)]
struct SingleQuoteContext { quote: Quote }

impl SingleQuoteContext {
    pub fn raw(id: i32, conn: &db::Conn) -> Option<SingleQuoteContext> {
        let quote = Quote::get_with_id(id, conn);
        if quote.is_err() { None }
        else { Some(SingleQuoteContext { quote: quote.unwrap() }) }
    }
}

#[derive(Debug, Serialize)]
struct QuoteListContext { quotes: Vec<Quote> }

impl QuoteListContext {
    pub fn raw(conn: &db::Conn) -> QuoteListContext {
        QuoteListContext { quotes: Quote::all(conn) }
    }
}


#[get("/")]
fn index(conn: db::Conn) -> Template {
    Template::render("index", &QuoteListContext::raw(&conn))
}

#[get("/quote/<id>")]
fn quote(id: i32, conn: db::Conn) -> Option<Template> {
    match SingleQuoteContext::raw(id, &conn) {
        Some(context) => Some(Template::render("quote", &context)),
        None => None
    }
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
        .manage(db::init_pool())
        .mount("/", routes![index, quote, files])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
