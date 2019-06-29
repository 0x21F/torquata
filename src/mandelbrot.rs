use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use color_scaling::scale_rgb;

pub fn mandelbrot(dimension_x: u32, dimension_y: u32, file_path: &str){
    let (re,im) = (0.285,0.01);
    
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(dimension_x, dimension_y);
    let max_iter = 1000; 
    
    let high: Rgb<u8> = Rgb([106,255,206]);
    let low: Rgb<u8> = Rgb([30,30,30]);

    for x in 0..dimension_x {
        for y in 0..dimension_y {
            let cx = (x as f32 / dimension_x as f32) * 3.5 - 2.5;
            let cy = (y as f32 / dimension_y as f32) * 2.0 - 1.0;

            let c = Complex::new(re ,im);
            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < max_iter && z.norm_sqr() < 2.0{
                z = z * z + c;
                i += 1;
            }
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = scale_rgb(&low, &high, (i as f64)/max_iter as f64).unwrap();
        }
    }
    imgbuf.save(file_path).unwrap();
} 

