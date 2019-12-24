#![feature(proc_macro_hygiene, decl_macro)]

pub mod security;
mod database;
mod views;

#[macro_use]
extern crate rocket;

extern crate rocket_client_addr;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn main() {
    let routes = routes![
        views::index,
        views::search_post,
        views::search_get,
        views::register,
        views::register_post,
        views::register_complete,
    ];

    rocket::ignite()
        .mount("/", routes)
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .launch();
}
