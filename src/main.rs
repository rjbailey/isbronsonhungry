extern crate env_logger;
extern crate iron;
extern crate mount;
extern crate openssl;
extern crate postgres;
extern crate staticfile;

use std::env;
use std::path::Path;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use openssl::ssl::{SslContext, SslMethod};
use postgres::{Connection, SslMode};
use staticfile::Static;

fn hello(_: &mut Request) -> IronResult<Response> {
    let ctx = SslContext::new(SslMethod::Sslv23).unwrap();
    let conn = Connection::connect(&env::var("DATABASE_URL").unwrap()[..],
                                   SslMode::Require(&ctx)).unwrap();
    let mut resp = Response::with((status::Ok, "Hello world!"));
    for row in &conn.query("SELECT 42", &[]).unwrap() {
        let result: i32 = row.get(0);
        resp = Response::with((status::Ok, result.to_string()));
    }
    Ok(resp)
}

fn get_server_port() -> u16 {
    env::var("PORT").ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    // Initialize the logger so we can see error messages.
    env_logger::init().unwrap();

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("static")));
    mount.mount("/hello", hello);

    // Run the server.
    Iron::new(mount).http(("0.0.0.0", get_server_port())).unwrap();
}
