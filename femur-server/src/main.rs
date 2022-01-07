use argon2::{PasswordHash, PasswordVerifier};
use clap::{App, Arg};
use std::error::Error;
use tiny_http::{Header, Method, Request, Response, Server, SslConfig, StatusCode};

fn not_implemented(req: Request, message: &str) -> Result<(), Box<dyn Error>> {
    req.respond(Response::from_string(message).with_status_code(StatusCode(501)))?;
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

fn authorize_and_authenticate_user(req: &Request) -> Result<bool, Box<dyn Error>> {
    let auth_headers = req
        .headers()
        .iter()
        .filter(|h| h.field.equiv("Authorization"))
        .collect::<Vec<&Header>>();
    let auth_header = auth_headers.get(0);
    if auth_header.is_none() {
        return Ok(false);
    }
    let auth_value = auth_header.unwrap().value.to_string();
    if !auth_value.starts_with("Basic ") {
        return Ok(false);
    }
    let decoded_credentials = base64::decode(auth_value.trim_start_matches("Basic "))?;
    let credentials = std::str::from_utf8(&decoded_credentials)?
        .split(':')
        .collect::<Vec<&str>>();
    if credentials.len() != 2 {
        return Ok(false);
    }
    let (username, password) = (credentials[0], credentials[1]);
    let auth_db = std::fs::read_to_string("./auth-db")?;
    if !auth_db.lines().any(|l| l.starts_with(username)) {
        return Ok(false);
    }
    let unparsed_hash = auth_db
        .lines()
        .find(|l| l.starts_with(username))
        .unwrap()
        .trim_start_matches(username)
        .trim_start();
    let parsed_hash = PasswordHash::new(unparsed_hash).unwrap();
    Ok(argon2::Argon2::new(
        argon2::Algorithm::Argon2i,
        argon2::Version::default(),
        argon2::Params::default(),
    )
    .verify_password(password.as_bytes(), &parsed_hash)
    .is_ok())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = App::new("femur-server")
        .arg(
            Arg::new("no-tls")
                .help("Disable TLS and send all traffic unencrypted")
                .takes_value(false)
                .long("no-tls"),
        )
        .arg(
            Arg::new("tls-cert")
                .help("Path to TLS certificate file")
                .takes_value(true)
                .long("tls-cert")
                .required_unless_present("no-tls")
                .conflicts_with("no-tls"),
        )
        .arg(
            Arg::new("tls-key")
                .help("Path to TLS key file")
                .takes_value(true)
                .long("tls-key")
                .required_unless_present("no-tls")
                .conflicts_with("no-tls"),
        )
        .get_matches();
    // TODO: make listening address and port configurable
    // TODO: handle error cases gracefully
    let server = if args.is_present("no-tls") {
        println!("Found --no-tls, running in plaintext mode --- ALL TRAFFIC IS UNENCRYPTED");
        Server::http("0.0.0.0:23856").unwrap()
    } else {
        let ssl_config = SslConfig {
            certificate: std::fs::read(std::path::Path::new(&args.value_of("tls-cert").unwrap()))?,
            private_key: std::fs::read(std::path::Path::new(&args.value_of("tls-key").unwrap()))?,
        };
        Server::https("0.0.0.0:23856", ssl_config).unwrap()
    };
    loop {
        match server.recv()? {
            mut r if r.url().starts_with("/fmrl/users") => {
                match r.method() {
                    Method::Get => {
                        if r.url().contains('?') {
                            // TODO: implement batch query
                            not_implemented(r, "Batch queries are not implemented (yet)")?;
                        } else {
                            let username = r.url().trim_start_matches("/fmrl/users/");
                            match get_status_by_user(username) {
                                // single user query
                                Ok(status) => {
                                    r.respond(Response::from_string(status.to_string()))?
                                }
                                // TODO: improve error handling
                                Err(e) => r.respond(
                                    Response::from_string(e.to_string())
                                        .with_status_code(StatusCode(500)),
                                )?,
                            }
                        }
                    }
                    Method::Put => {
                        if r.url().ends_with("avatar") {
                            // TODO: implement setting avatar
                            not_implemented(r, "Avatar updates are not implemented (yet)")?;
                        } else {
                            // set status field(s)
                            if authorize_and_authenticate_user(&r)? {
                                let username =
                                    r.url().trim_start_matches("/fmrl/users/").to_string();
                                let mut update_string = String::new();
                                r.as_reader().read_to_string(&mut update_string)?;
                                let update = update_string.parse()?;
                                set_status_by_user(&username, update)?;
                                r.respond(Response::empty(StatusCode(200)))?;
                            } else {
                                r.respond(Response::empty(StatusCode(401)))?;
                            }
                        }
                    }
                    _ => r.respond(
                        Response::from_string("Method not allowed for '/fmrl/users'")
                            .with_status_code(StatusCode(405)),
                    )?,
                }
            }
            r if r.method() == &Method::Get && r.url().starts_with("/fmrl/new") => {
                // TODO: implement delta query
                not_implemented(r, "Delta queries are not implemented (yet)")?;
            }
            r => r.respond(
                Response::from_string("Invalid path or method").with_status_code(StatusCode(400)),
            )?,
        };
    }
}
