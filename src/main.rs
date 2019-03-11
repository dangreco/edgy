#[macro_use]
extern crate clap;
extern crate image;
extern crate pbr;

use clap::App;

use std::fs;
use pbr::ProgressBar;
use image::{
    GenericImageView,
    ImageBuffer,
    Luma
};


fn sobel(input: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>>
{
    let width: u32 = input.width() - 2;
    let height: u32 = input.height() - 2;
    let mut buff: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for i in 0..width {
        for j in 0..height {
            /* Unwrap those loops! */
            let val0 = input.get_pixel(i, j).data[0] as i32;
            let val1 = input.get_pixel(i + 1 , j).data[0] as i32;
            let val2 = input.get_pixel(i + 2, j).data[0] as i32;
            let val3 = input.get_pixel(i, j + 1).data[0] as i32;
            let val5 = input.get_pixel(i + 2, j + 1).data[0] as i32;
            let val6 = input.get_pixel(i, j + 2).data[0] as i32;
            let val7 = input.get_pixel(i + 1, j + 2).data[0] as i32;
            let val8 = input.get_pixel(i + 2, j + 2).data[0] as i32;
            /* Apply Sobel kernels */
            let gx = (-1 * val0) + (-2 * val3) + (-1 * val6) + val2 + (2 * val5) + val8;
            let gy = (-1 * val0) + (-2 * val1) + (-1 * val2) + val6 + (2 * val7) + val8;
            let mut mag = ((gx as f64).powi(2) + (gy as f64).powi(2)).sqrt();

            if mag > 255.0 {
                mag = 255.0;
            }

            buff.put_pixel(i, j, Luma([mag as u8]));
        }
    }

    return buff;
}


fn sigma(width: u32, height: u32, blur_modifier: i32) -> f32
{
    return (((width * height) as f32) / 3630000.0) * blur_modifier as f32;
}


fn process_frame(path: String, output_path: String, blur_modifier: i32)
{
    let source = image::open(path).unwrap();
    let (width, height) = source.dimensions();
    let sigma = sigma(width, height, blur_modifier);
    let gaussed = source.blur(sigma);
    let gray = gaussed.to_luma();
    let sobeled = sobel(&gray);
    sobeled.save(output_path).unwrap();
}


fn process_multiple(source_dir: String, output_dir: String, blur_modifier: i32)
{
    let mut paths = fs::read_dir(&source_dir).unwrap();
    let total = paths.count() as u64;
    paths = fs::read_dir(&source_dir).unwrap();
    let mut bar = ProgressBar::new(total);

    for path in paths {
        let path_str = path.unwrap().path().display().to_string();
        let frame_number = &path_str[path_str.len() - 8..path_str.len()-4].to_string();
        let output_str = format!("frame{}.bmp", frame_number);
        process_frame(path_str, format!("{}/{}", output_dir, output_str), blur_modifier);
        bar.inc();
    }
}


fn main()
{
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("INPUT").unwrap().to_string();
    let output = matches.value_of("OUTPUT").unwrap().to_string();
    let blur_mod = matches.value_of("BLUR").unwrap_or("1").parse::<i32>().unwrap();

    if matches.is_present("MULTIPLE") {
        process_multiple(input, output, blur_mod);
    } else {
        process_frame(input, output, blur_mod);
    }

}