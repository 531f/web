#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use mysql;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::net::SocketAddr;

static SQL_URI: &'static str = "mysql://rust:admin1234@192.168.1.91:3306/Web";

#[derive(FromForm)]
struct SearchText {
    name: String,
    surname: String,
}

#[derive(FromForm)]
struct Register {
    name: String,
    surname: String,
    email: String,
    interests: String,
}

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    surname: String,
    name: String,
    image: String,
}

#[get("/register_complete")]
fn register_complete() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("register_complete", &map)
}

#[post("/register_post", data="<register_data>")]
fn register_post(remote_addr: SocketAddr, register_data: Form<Register>) -> Redirect {
    let pool = mysql::Pool::new(SQL_URI).unwrap();

    let query: String;
    if register_data.interests.is_empty() {
        query = format!("INSERT IGNORE INTO Comrades (IP, Name, Surname, Email) VALUES ('{}', '{}', '{}', '{}')", remote_addr.ip().to_string(), register_data.name, register_data.surname, register_data.email);
    } else {
        query = format!("INSERT IGNORE INTO Comrades (IP, Name, Surname, Email, Interests) VALUES ('{}', '{}', '{}', '{}', '{}')", remote_addr.ip().to_string(), register_data.name, register_data.surname, register_data.email, register_data.interests);
    }

    pool.prep_exec(query, ()).unwrap();
    Redirect::to("register_complete")
}

#[get("/register")]
fn register() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("register", &map)
}

#[get("/data")]
fn data() -> &'static str {
    "This is the database of seif.es"
}

#[get("/search")]
fn search_get() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("search", &map)
}

#[post("/search", data = "<search_text>")]
fn search_post(remote_addr: SocketAddr, search_text: Form<SearchText>) -> Template {
    let mut map = HashMap::new();
    let mut people: Vec<HashMap<String, String>> = Vec::new();

    let pool = mysql::Pool::new(SQL_URI).unwrap();

    pool.prep_exec(format!("INSERT IGNORE INTO Search_History (IP, Name, Surname) VALUES ('{}', '{}', '{}')", remote_addr.ip().to_string(), search_text.name, search_text.surname), ()).unwrap();

    let users: Vec<User> = pool
        .prep_exec(format!("SELECT * FROM Users WHERE name LIKE '%{}%' AND surname LIKE '%{}%' LIMIT 50", search_text.name, search_text.surname), ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, surname, name, image) = mysql::from_row(row);
                    User {
                        id: id,
                        surname: surname,
                        name: name,
                        image: image,
                    }
                })
                .collect()
        })
        .unwrap();

    for user in users {
        let mut user_clean: HashMap<String, String> = HashMap::new();
        user_clean.insert("name".to_string(), format!("{} {}", user.name, user.surname));
        user_clean.insert("image".to_string(), user.image.to_string());

        people.push(user_clean);
    }

    map.insert("people", people);
    Template::render("search", &map)
}

#[get("/")]
fn index() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("index", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, data, register, search_post, search_get, register_post, register_complete])
        .mount("/public", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
