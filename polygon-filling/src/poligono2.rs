use image::{Rgb, RgbImage};

use crate::fill::scanline_fill;
use crate::line::draw_polygon_border;

pub fn points() -> Vec<(i32, i32)> {
    vec![(321, 335), (288, 286), (339, 251), (374, 302)]
}

pub fn draw(img: &mut RgbImage) {
    let fill_color = Rgb([0, 0, 255]);
    let border_color = Rgb([255, 255, 255]);

    let polygon = points();
    scanline_fill(img, &polygon, fill_color, &[]);
    draw_polygon_border(img, &polygon, border_color);
}
