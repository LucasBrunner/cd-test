use std::net::Ipv4Addr;

use rocket::Config;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[launch]
fn rocket() -> _ {
  println!("Starting server!");
  let config = Config {
    port: 8000,
    address: Ipv4Addr::new(0, 0, 0, 0).into(),
    ..Config::debug_default()
  };

  rocket::custom(&config).mount("/", routes![index])
}