use image::{Rgb, RgbImage};

pub fn set_pixel(img: &mut RgbImage, x: i32, y: i32, color: Rgb<u8>) {
    let (width, height) = img.dimensions();
    if x >= 0 && y >= 0 && (x as u32) < width && (y as u32) < height {
        img.put_pixel(x as u32, y as u32, color);
    }
}

pub fn bresenham_line(img: &mut RgbImage, p0: (i32, i32), p1: (i32, i32), color: Rgb<u8>) {
    let (mut x0, mut y0) = p0;
    let (x1, y1) = p1;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        set_pixel(img, x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_polygon_border(img: &mut RgbImage, points: &[(i32, i32)], color: Rgb<u8>) {
    let n = points.len();
    for i in 0..n {
        let p0 = points[i];
        let p1 = points[(i + 1) % n];
        bresenham_line(img, p0, p1, color);
    }
}
