#[macro_use] extern crate serde;
extern crate image;
extern crate rayon;

mod mandelbrot;
use mandelbrot::FractalType;

fn main() {
    mandelbrot::julia(mandelbrot::Fractal{
        fractal_type: FractalType::Julia(-0.79, 0.15),
        dim_x: 3840,
        dim_y: 2160,
        path: "julia_test.png".to_string(),
    });
}

