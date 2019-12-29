use crate::database;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

#[derive(Default)]
pub struct AccessLogger {}

impl Fairing for AccessLogger {
    fn info(&self) -> Info {
        Info {
            name: "Access logger",
            kind: Kind::Request,
        }
    }

    // Capture all requests and insert into database
    fn on_request(&self, request: &mut Request, _: &Data) {
        match request.real_ip() {
            Some(val) => database::insert_access_log(
                &val.to_string(),
                &String::from(request.uri().path()),
            ),
            None => {
                database::insert_app_log(
                    database::LogType::ERROR,
                    &String::from("Failed to get IP on access logger"),
                );
                database::insert_access_log(
                    &String::from("?.?.?.?"),
                    &String::from(request.uri().path()),
                );
            },
        }
    }
}
