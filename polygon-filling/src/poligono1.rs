use image::{Rgb, RgbImage};

use crate::fill::scanline_fill;
use crate::line::draw_polygon_border;

pub fn points() -> Vec<(i32, i32)> {
    vec![
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ]
}

pub fn draw(img: &mut RgbImage) {
    let fill_color = Rgb([255, 255, 0]);
    let border_color = Rgb([255, 255, 255]);

    let polygon = points();
    scanline_fill(img, &polygon, fill_color, &[]);
    draw_polygon_border(img, &polygon, border_color);
}
