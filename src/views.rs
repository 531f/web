use crate::database;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_client_addr::ClientRealAddr;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(FromForm)]
pub struct Search {
    name: String,
    surname: String,
}

#[derive(FromForm)]
pub struct Register {
    name: String,
    surname: String,
    email: String,
    interests: String,
}

#[get("/register_complete")]
pub fn register_complete() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("register_complete", &map)
}

#[post("/register_post", data = "<register_data>")]
pub fn register_post(remote_addr: &ClientRealAddr, register_data: Form<Register>) -> Redirect {
    let remote_addr_string = remote_addr
        .get_ipv4_string()
        .unwrap();
    
    let interests = match register_data.interests.is_empty() {
        true => None,
        false => Some(&register_data.interests),
    };

    database::insert_comrade(
        &remote_addr_string,
        &register_data.name,
        &register_data.surname,
        &register_data.email,
        interests
    );

    Redirect::to("register_complete")
}

#[get("/register")]
pub fn register() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("register", &map)
}

#[get("/search")]
pub fn search_get() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("search", &map)
}

#[post("/search", data = "<search_data>")]
pub fn search_post(remote_addr: &ClientRealAddr, search_data: Form<Search>) -> Template {
    let remote_addr_string = remote_addr.get_ipv4_string().unwrap();
    
    database::insert_search(&remote_addr_string, &search_data.name, &search_data.surname);
    let users: Vec<database::User> = match database::get_users(&search_data.name, &search_data.surname) {
        Some(val) => val,
        None => Vec::new()
    };

    let mut map: HashMap<String, Vec<database::User>> = HashMap::new();
    map.insert(String::from("users"), users);
    Template::render("search", &map)
}

#[get("/")]
pub fn index() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("index", &map)
}
