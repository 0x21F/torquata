#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate bcrypt;
extern crate chrono;
extern crate env_logger;
extern crate ratelimit;
extern crate image;
mod mandelbrot;

fn main() {
    mandelbrot::mandelbrot(3840,2160,"mandelbrot.png");
    mandelbrot::julia(3840,2160,"julia.png");
}
