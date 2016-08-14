#![feature(plugin, custom_derive)]
#![plugin(postgres_derive_macros, serde_macros)]

extern crate chrono;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate mount;
extern crate num_cpus;
extern crate openssl;
#[macro_use]
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
extern crate staticfile;

use std::env;
use std::path::Path;

use chrono::{DateTime, UTC};
use iron::prelude::*;
use iron::status;
use mount::Mount;
use openssl::ssl::{SslContext, SslMethod};
use postgres::rows::Row;
use r2d2_postgres::PostgresConnectionManager as PCM;
use r2d2_postgres::SslMode;
use staticfile::Static;

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
enum Activity {
    Feeding,
    Petting,
    Playing,
    Talking,
}

#[derive(Debug, Serialize)]
struct Event {
    id: i32,
    activity: Activity,
    human: String,
    time: DateTime<UTC>,
}

impl Event {
    fn from_row(row: &Row) -> Self {
        Event {
            id: row.get("id"),
            activity: row.get("activity"),
            human: row.get("human"),
            time: row.get("time"),
        }
    }
}

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

fn get_events(_: &mut Request) -> IronResult<Response> {
    let conn = DB_POOL.get().unwrap();
    let rows = &conn.query("SELECT * FROM events", &[]).unwrap();
    let events: Vec<_> = rows.iter().map(|row| Event::from_row(&row)).collect();
    Ok(Response::with((status::Ok, serde_json::to_string(&events).unwrap())))
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
    mount.mount("/events", get_events);

    // Run the server.
    Iron::new(mount).http(("0.0.0.0", get_server_port())).unwrap();
}
