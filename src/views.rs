use crate::security::sanitize;
use mysql;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_client_addr::ClientRealAddr;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

static SQL_URI: &'static str = "mysql://rust:admin1234@192.168.1.91:3306/Web";

#[derive(FromForm)]
pub struct SearchText {
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

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    id: i32,
    surname: String,
    name: String,
    image: String,
}

#[get("/register_complete")]
pub fn register_complete() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("register_complete", &map)
}

#[post("/register_post", data = "<register_data>")]
pub fn register_post(remote_addr: &ClientRealAddr, register_data: Form<Register>) -> Redirect {
    let pool = mysql::Pool::new(SQL_URI).unwrap();
    let remote_addr_string = remote_addr.get_ipv4_string().unwrap();

    let name = sanitize(&register_data.name);
    let surname = sanitize(&register_data.surname);
    let email = sanitize(&register_data.email);

    let query: String;
    if register_data.interests.is_empty() {
        query = format!("INSERT IGNORE INTO Comrades (IP, Name, Surname, Email) VALUES ('{}', '{}', '{}', '{}')", remote_addr_string, name, surname, email);
    } else {
        let interests = sanitize(&register_data.interests);
        query = format!("INSERT IGNORE INTO Comrades (IP, Name, Surname, Email, Interests) VALUES ('{}', '{}', '{}', '{}', '{}')", remote_addr_string, name, surname, email, interests);
    }

    pool.prep_exec(query, ()).unwrap();
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

#[post("/search", data = "<search_text>")]
pub fn search_post(remote_addr: &ClientRealAddr, search_text: Form<SearchText>) -> Template {
    let mut map = HashMap::new();
    let mut people: Vec<HashMap<String, String>> = Vec::new();
    let remote_addr_string = remote_addr.get_ipv4_string().unwrap();

    let pool = mysql::Pool::new(SQL_URI).unwrap();
    let name = sanitize(&search_text.name);
    let surname = sanitize(&search_text.surname);

    pool.prep_exec(
        format!(
            "INSERT IGNORE INTO Search_History (IP, Name, Surname) VALUES ('{}', '{}', '{}')",
            remote_addr_string, name, surname
        ),
        (),
    )
    .unwrap();

    let users: Vec<User> = pool
        .prep_exec(
            format!(
                "SELECT * FROM Users WHERE name LIKE '%{}%' AND surname LIKE '%{}%' LIMIT 50",
                name, surname
            ),
            (),
        )
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
        user_clean.insert(
            "name".to_string(),
            format!("{} {}", user.name, user.surname),
        );
        user_clean.insert("image".to_string(), user.image.to_string());
        user_clean.insert("id".to_string(), user.id.to_string());
        people.push(user_clean);
    }

    map.insert("people", people);
    Template::render("search", &map)
}

#[get("/")]
pub fn index() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("index", &map)
}
