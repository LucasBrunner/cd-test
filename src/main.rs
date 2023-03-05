use std::{fmt::Write, net::Ipv4Addr};

use rocket::Config;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/fizzbuzz/index/<index>")]
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

#[cfg(test)]
mod fizzbuzz_index_tests {
  use super::fizzbuzz_index;

  #[test]
  fn test_fizz() {
    assert_eq!(fizzbuzz_index(3), "Fizz");
    assert_eq!(fizzbuzz_index(9), "Fizz");
    assert_eq!(fizzbuzz_index(21), "Fizz");
  }

  #[test]
  fn test_buzz() {
    assert_eq!(fizzbuzz_index(5), "Buzz");
    assert_eq!(fizzbuzz_index(20), "Buzz");
    assert_eq!(fizzbuzz_index(35), "Buzz");
  }

  #[test]
  fn test_fizzbuzz() {
    assert_eq!(fizzbuzz_index(15), "FizzBuzz");
    assert_eq!(fizzbuzz_index(30), "FizzBuzz");
    assert_eq!(fizzbuzz_index(75), "FizzBuzz");
  }

  #[test]
  fn test_num() {
    assert_eq!(fizzbuzz_index(2), "2");
    assert_eq!(fizzbuzz_index(7), "7");
    assert_eq!(fizzbuzz_index(14), "14");
  }
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
