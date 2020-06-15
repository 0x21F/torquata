use color_scaling::scale_rgb;
use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::Result;
#[derive(Deserialize)]
pub enum FractalType{
    Julia(f64,f64),
    Mandelbrot,
    Tricorn,
}

#[derive(Deserialize)]
pub struct Fractal {
    pub fractal_type: FractalType,
    pub dim_x: u32,
    pub dim_y: u32,
    pub path: String,
}


pub fn julia(frac: Fractal) -> Result<()>{
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(frac.dim_x, frac.dim_y);
    let max_iter = 1000;
    let (re, im);
    
    if let FractalType::Julia(real,imaginary) = frac.fractal_type{
        re = real;
        im = imaginary;
    }
    else {
        re = -0.79;
        im = 0.15;
    }
    let high: Rgb<u8> = Rgb([106,255,206]);
    let low: Rgb<u8> = Rgb([5,5,5]);
    imgbuf.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pix)|{
            let cx = (x as f64 / frac.dim_x as f64) * 3.5 - 2.5;
            let cy = (y as f64 / frac.dim_y as f64) * 2.0 - 1.0;

            let c = Complex::new(re ,im);
            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < max_iter && z.norm_sqr() <= 4.0{
                z = z * z + c;
                i += 1;
            }
            if i < max_iter {
                *pix = scale_rgb(&low, &high, (i as f64)/max_iter as f64).unwrap();
            }
            else {
                *pix = low;
            }
    });
    imgbuf.save(frac.path)
} 

pub fn mandelbrot(frac: Fractal) -> Result<()>{
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(frac.dim_x, frac.dim_y);
    let max_iter = 1000;

    let high: Rgb<u8> = Rgb([106,255,206]);
    let low: Rgb<u8> = Rgb([5,5,5]);

    imgbuf.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pix)|{
            let cx = (x as f64 / frac.dim_x as f64) * 3.5 - 2.5;
            let cy = (y as f64 / frac.dim_y as f64) * 2.0 - 1.0;

            let mut x = 0.0_f64;
            let mut y = 0.0_f64;
            
            let mut i = 0;
            while i < max_iter && x * x + y * y <= 4.0{
                let xtemp = x * x - y * y + cx;
                y = 2.0 * x * y + cy;
                x = xtemp;
                i += 1;
            }
            if i < max_iter {
                *pix = scale_rgb(&low, &high, (i as f64)/max_iter as f64).unwrap();
            }
            else {
                *pix = low;
            }
    });
    imgbuf.save(frac.path)
} 

pub fn tricorn(frac: Fractal) -> Result<()>{
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(frac.dim_x, frac.dim_y);
    let max_iter = 1000;

    let high: Rgb<u8> = Rgb([106,255,206]);
    let low: Rgb<u8> = Rgb([5,5,5]);

    imgbuf.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pix)|{
            let cx = (x as f64 / frac.dim_x as f64) * 3.5 - 2.5;
            let cy = (y as f64 / frac.dim_y as f64) * 2.0 - 1.0;

            let mut x = 0.0_f64;
            let mut y = 0.0_f64;
            
            let mut i = 0;
            while i < max_iter && x * x + y * y <= 4.0{
                let xtemp = x * x - y * y + cx;
                y = -2.0 * x * y + cy;
                x = xtemp;
                i += 1;
            }
            if i < max_iter {
                *pix = scale_rgb(&low, &high, (i as f64)/max_iter as f64).unwrap();
            }
            else {
                *pix = low;
            }
    });
    imgbuf.save(frac.path)
} 
