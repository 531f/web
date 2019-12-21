use std::string::String;

pub fn sanitize(user_input: &String) -> String {
    let allowed_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let clean: Vec<Option<char>> = user_input
        .chars()
        .map(|c| {
            if allowed_chars.contains(&c) {
                Some(c)
            }
            else {
                None
            }
        })
        .collect();

    let mut clean_string = String::new();

    for o in clean {
        match o {
            Some(c) => clean_string.push(c),
            None => {}
        }
    }

    clean_string
}