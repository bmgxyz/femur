use std::error::Error;

fn not_implemented(req: tiny_http::Request, message: &str) -> Result<(), Box<dyn Error>> {
    req.respond(
        tiny_http::Response::from_string(message).with_status_code(tiny_http::StatusCode(501)),
    )?;
    Ok(())
}

fn get_status_by_user(username: &str) -> Result<femur::UserData, Box<dyn Error>> {
    let status_path = std::path::Path::new(username);
    if status_path.exists() {
        Ok(std::fs::read_to_string(status_path)?.parse()?)
    } else {
        Err(format!(
            "Username '{}' does not have a status on this server",
            username
        )
        .into())
    }
}

fn set_status_by_user(
    username: &str,
    status_updates: femur::UserData,
) -> Result<(), Box<dyn Error>> {
    let status_path = std::path::Path::new(username);
    let new_status = if status_path.exists() {
        let old_status: femur::UserData = std::fs::read_to_string(status_path)?.parse()?;
        old_status.update_status_fields(status_updates)
    } else {
        status_updates
    };
    std::fs::write(status_path, new_status.to_string())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: make listening address and port configurable
    // TODO: handle error cases gracefully
    // TODO: enable TLS
    let server = tiny_http::Server::http("0.0.0.0:23856").unwrap();
    loop {
        match server.recv()? {
            mut r if r.url().starts_with("/fmrl/users") => {
                match r.method() {
                    tiny_http::Method::Get => {
                        if r.url().contains('?') {
                            // TODO: implement batch query
                            not_implemented(r, "Batch queries are not implemented (yet)")?;
                        } else {
                            let username = r.url().trim_start_matches("/fmrl/users/");
                            match get_status_by_user(username) {
                                // single user query
                                Ok(status) => {
                                    r.respond(tiny_http::Response::from_string(status.to_string()))?
                                }
                                // TODO: improve error handling
                                Err(e) => r.respond(
                                    tiny_http::Response::from_string(e.to_string())
                                        .with_status_code(tiny_http::StatusCode(500)),
                                )?,
                            }
                        }
                    }
                    tiny_http::Method::Put => {
                        if r.url().ends_with("avatar") {
                            // TODO: implement setting avatar
                            not_implemented(r, "Avatar updates are not implemented (yet)")?;
                        } else {
                            // set status field(s)
                            let username = r.url().trim_start_matches("/fmrl/users/").to_string();
                            let mut update_string = String::new();
                            r.as_reader().read_to_string(&mut update_string)?;
                            let update = update_string.parse()?;
                            set_status_by_user(&username, update)?;
                            r.respond(tiny_http::Response::empty(tiny_http::StatusCode(200)))?;
                        }
                    }
                    _ => r.respond(
                        tiny_http::Response::from_string("Method not allowed for '/fmrl/users'")
                            .with_status_code(tiny_http::StatusCode(405)),
                    )?,
                }
            }
            r if r.method() == &tiny_http::Method::Get && r.url().starts_with("/fmrl/new") => {
                // TODO: implement delta query
                not_implemented(r, "Delta queries are not implemented (yet)")?;
            }
            r => r.respond(
                tiny_http::Response::from_string("Invalid path or method")
                    .with_status_code(tiny_http::StatusCode(400)),
            )?,
        };
    }
}
