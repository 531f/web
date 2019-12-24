use crate::security::sanitize;
use mysql;

static SQL_URI: &'static str = "mysql://rust:admin1234@192.168.1.91:3306/Web";

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub surname: String,
    pub name: String,
    pub image: String,
}

fn exec_set(query: String) -> Result<(), String> {
    let pool = mysql::Pool::new(SQL_URI).unwrap();
    pool.prep_exec(query, ()).unwrap();

    Ok(())
}

pub fn insert_comrade(ip: &String, name: &String, surname: &String, email: &String, interests: Option<&String>) {
    let query = match interests {
        None => format!("INSERT IGNORE INTO Comrades (IP, Name, Surname, Email) VALUES ('{}', '{}', '{}', '{}')",
            ip,
            sanitize(&name),
            sanitize(&surname),
            sanitize(&email),
        ),
        Some(val) => format!("INSERT IGNORE INTO Comrades (IP, Name, Surname, Email, Interests) VALUES ('{}', '{}', '{}', '{}', '{}')",
            ip,
            sanitize(&name),
            sanitize(&surname),
            sanitize(&email),
            sanitize(&val),
        ),
    };

    exec_set(query).unwrap();
}

pub fn insert_search(ip: &String, name: &String, surname: &String) {
    let query = format!("INSERT IGNORE INTO Search_History (IP, Name, Surname) VALUES ('{}', '{}', '{}')",
        ip,
        sanitize(&name),
        sanitize(&surname),
    );

    exec_set(query).unwrap();
}

pub fn get_users(name: &String, surname: &String) -> Option<Vec<User>> {
    let query = format!("SELECT * FROM Users WHERE name LIKE '%{}%' AND surname LIKE '%{}%' LIMIT 50",
        sanitize(&name),
        sanitize(&surname),
    );

    let pool = mysql::Pool::new(SQL_URI).unwrap();
    let res = pool.prep_exec(query, (),)
        .map(|result| {
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