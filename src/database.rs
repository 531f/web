use crate::security::sanitize;
use mysql;
use serde::ser::{Serialize, SerializeStruct, Serializer};

static SQL_URI: &'static str = "mysql://rust:admin1234@192.168.1.91:3306/Web";

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub surname: String,
    pub name: String,
    pub image: String,
}

pub enum LogType {
    INFO,
    WARNING,
    DEBUG,
    ERROR,
    CRITICAL,
}

impl LogType {
    pub fn as_str(&self) -> &str {
        match self {
            LogType::INFO => "INFO",
            LogType::WARNING => "WARNING",
            LogType::DEBUG => "DEBUG",
            LogType::ERROR => "ERROR",
            LogType::CRITICAL => "CRITICAL",
        }
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("surname", &self.surname)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("image", &self.image)?;
        state.end()
    }
}

fn exec_set(query: String) {
    match mysql::Pool::new(SQL_URI) {
        Ok(pool) => {
            match pool.prep_exec(query.clone(), ()) {
                Ok(_) => {}
                Err(_) => insert_app_log(
                    LogType::ERROR,
                    &String::from(format!("Failed to execute query {}", query)),
                ),
            };
        }
        Err(e) => insert_app_log(
            LogType::CRITICAL,
            &String::from(format!("Failed to connect to database [{}]", e)),
        ),
    };
}

pub fn insert_comrade(
    ip: &String,
    name: &String,
    surname: &String,
    email: &String,
    interests: Option<&String>,
) {
    let query = match interests {
        None => format!("INSERT IGNORE INTO comrades (IP, Name, Surname, Email) VALUES ('{}', '{}', '{}', '{}')",
            ip,
            sanitize(&name),
            sanitize(&surname),
            sanitize(&email),
        ),
        Some(val) => format!("INSERT IGNORE INTO comrades (IP, Name, Surname, Email, Interests) VALUES ('{}', '{}', '{}', '{}', '{}')",
            ip,
            sanitize(&name),
            sanitize(&surname),
            sanitize(&email),
            sanitize(&val),
        ),
    };

    exec_set(query)
}

pub fn insert_search(ip: &String, name: &String, surname: &String) {
    let query = format!(
        "INSERT IGNORE INTO search_history (IP, Name, Surname) VALUES ('{}', '{}', '{}')",
        ip,
        sanitize(&name),
        sanitize(&surname),
    );

    exec_set(query)
}

pub fn insert_app_log(log_type: LogType, msg: &String) {
    let query = format!(
        "INSERT IGNORE INTO app_log (Type, Msg) VALUES ('{}', '{}')",
        log_type.as_str(),
        sanitize(&msg),
    );

    exec_set(query)
}

pub fn insert_access_log(ip: &String, page: &String) {
    let query = format!(
        "INSERT IGNORE INTO access_log (Ip, Page) VALUES ('{}', '{}')",
        ip,
        sanitize(&page),
    );

    exec_set(query)
}

pub fn get_people(name: &String, surname: &String) -> Option<Vec<User>> {
    let query = format!(
        "SELECT * FROM uam_people WHERE name LIKE '%{}%' AND surname LIKE '%{}%' LIMIT 50",
        sanitize(&name),
        sanitize(&surname),
    );

    let pool = mysql::Pool::new(SQL_URI).unwrap();
    let res = pool.prep_exec(query, ()).map(|result| {
        result
            .map(|row| {
                let (id, surname, name, image) = mysql::from_row(row.unwrap());
                User {
                    id: id,
                    surname: surname,
                    name: name,
                    image: image,
                }
            })
            .collect()
    });

    match res {
        Ok(val) => Some(val),
        Err(_) => {
            println!("Failed to get users");
            return None;
        }
    }
}
