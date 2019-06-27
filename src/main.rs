#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
extern crate bcrypt;
extern crate chrono;
extern crate env_logger;
extern crate ratelimit;

fn main() {
    println!("Hello, world!");
}
