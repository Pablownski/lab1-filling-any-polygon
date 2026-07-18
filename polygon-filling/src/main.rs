mod fill;
mod line;
mod poligono1;
mod poligono2;

use image::{Rgb, RgbImage};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 500;

fn main() {
    let mut img = RgbImage::from_pixel(WIDTH, HEIGHT, Rgb([0, 0, 0]));

    poligono1::draw(&mut img);
    poligono2::draw(&mut img);

    img.save("out.bmp").expect("failed to save image");
    println!("Saved out.bmp ({}x{})", WIDTH, HEIGHT);
}
