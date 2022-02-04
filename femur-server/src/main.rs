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
        .arg(
            Arg::new("listen")
                .help("Address to listen on")
                .takes_value(true)
                .long("listen")
                .default_value_if("no-tls", None, Some("0.0.0.0:80"))
                .default_value("0.0.0.0:443"),
        )
        .get_matches();
    // TODO: handle error cases gracefully
    let server = if args.is_present("no-tls") {
        println!("Found --no-tls, running in plaintext mode --- ALL TRAFFIC IS UNENCRYPTED");
        Server::http(args.value_of("listen").unwrap()).unwrap()
    } else {
        let ssl_config = SslConfig {
            certificate: std::fs::read(std::path::Path::new(&args.value_of("tls-cert").unwrap()))?,
            private_key: std::fs::read(std::path::Path::new(&args.value_of("tls-key").unwrap()))?,
        };
        Server::https(args.value_of("listen").unwrap(), ssl_config).unwrap()
    };
    loop {
        match server.recv()? {
            // robots.txt
            r if r.url().starts_with("/robots.txt") && r.method() == &Method::Get => {
                todo!();
            },
            // CORS compliance
            r if r.method() == &Method::Options => {
                todo!();
            },
            // status query
            r if r.url().starts_with("/.well-known/fmrl/users") && r.method() == &Method::Get => {
                todo!();
            }
            // set status field(s)
            r if r.url().starts_with("/.well-known/fmrl/user/") && r.method() == &Method::Patch => {
                todo!();
            }
            // set or delete avatar
            r if r.url().starts_with("/.well-known/fmrl/user/") && r.url().ends_with("/avatar") => {
                match r.method() {
                    // set avatar
                    Method::Put => {
                        todo!();
                    }
                    // delete avatar
                    Method::Delete => {
                        todo!();
                    }
                    _ => {
                        r.respond(
                            Response::from_string("Invalid method for avatar operations")
                                .with_status_code(StatusCode(400)),
                        )?;
                    }
                };
            }
            r if r.url().starts_with("/.well-known/fmrl/user/")
                && r.url().ends_with("/following") =>
            {
                match r.method() {
                    // get following
                    Method::Get => {
                        todo!();
                    }
                    // set following
                    Method::Patch => {
                        todo!();
                    }
                    _ => {
                        r.respond(
                            Response::from_string("Invalid method for following operations")
                                .with_status_code(StatusCode(405)),
                        )?;
                    }
                }
            }
            r => r.respond(
                Response::from_string("Invalid path or method").with_status_code(StatusCode(405)),
            )?,
        };
    }
}
