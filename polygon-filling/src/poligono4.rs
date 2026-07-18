use image::{Rgb, RgbImage};

use crate::fill::scanline_fill;
use crate::line::draw_polygon_border;

pub fn points() -> Vec<(i32, i32)> {
    vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ]
}

// Polygon 5 sits inside polygon 4's silhouette and acts as a hole: it is
// never filled, only its border is drawn.
fn hole_points() -> Vec<(i32, i32)> {
    vec![(682, 175), (708, 120), (735, 148), (739, 170)]
}

pub fn draw(img: &mut RgbImage) {
    let fill_color = Rgb([0, 255, 0]);
    let border_color = Rgb([255, 255, 255]);

    let polygon = points();
    let hole = hole_points();

    scanline_fill(img, &polygon, fill_color, &[&hole]);
    draw_polygon_border(img, &polygon, border_color);
    draw_polygon_border(img, &hole, border_color);
}
