use std::{fmt::Write, net::Ipv4Addr};

use rocket::Config;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/fizzbuzz/<index>")]
fn fizzbuzz_index(index: u32) -> String {
  let mut output = String::new();
  if index % 3 == 0 {
    _ = output.write_fmt(format_args!("Fizz"));
  }
  if index % 5 == 0 {
    _ = output.write_fmt(format_args!("Buzz"));
  }
  if output.len() == 0 {
    _ = output.write_fmt(format_args!("{index}"));
  }
  output
}

#[launch]
fn rocket() -> _ {
  let config = Config {
    port: 8000,
    address: Ipv4Addr::new(0, 0, 0, 0).into(),
    ..Config::debug_default()
  };

  rocket::custom(&config).mount("/", routes![index, fizzbuzz_index])
}
