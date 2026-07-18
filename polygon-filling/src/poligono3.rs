use image::{Rgb, RgbImage};

use crate::fill::scanline_fill;
use crate::line::draw_polygon_border;

pub fn points() -> Vec<(i32, i32)> {
    vec![(377, 249), (411, 197), (436, 249)]
}

pub fn draw(img: &mut RgbImage) {
    let fill_color = Rgb([255, 0, 0]);
    let border_color = Rgb([255, 255, 255]);

    let polygon = points();
    scanline_fill(img, &polygon, fill_color, &[]);
    draw_polygon_border(img, &polygon, border_color);
}
