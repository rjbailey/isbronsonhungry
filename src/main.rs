extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate mount;
extern crate num_cpus;
extern crate openssl;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate staticfile;

use std::env;
use std::path::Path;
use iron::prelude::*;
use iron::status;
use mount::Mount;
use openssl::ssl::{SslContext, SslMethod};
use r2d2_postgres::PostgresConnectionManager as PCM;
use r2d2_postgres::SslMode;
use staticfile::Static;

lazy_static! {
    static ref DB_POOL: r2d2::Pool<PCM> = {
        let ctx = Box::new(SslContext::new(SslMethod::Sslv23).unwrap());
        let url = env::var("DATABASE_URL").unwrap();
        let config = r2d2::Config::builder()
            .pool_size(num_cpus::get() as u32)
            .build();
        let manager = PCM::new(&url[..], SslMode::Require(ctx)).unwrap();
        r2d2::Pool::new(config, manager).unwrap()
    };
}

fn hello(_: &mut Request) -> IronResult<Response> {
    let conn = DB_POOL.get().unwrap();
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
