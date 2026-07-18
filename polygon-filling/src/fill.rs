use image::{Rgb, RgbImage};

use crate::line::set_pixel;

fn row_intersections(points: &[(i32, i32)], y: i32) -> Vec<f64> {
    let n = points.len();
    let mut intersections: Vec<f64> = Vec::new();

    for i in 0..n {
        let (mut x0, mut y0) = points[i];
        let (mut x1, mut y1) = points[(i + 1) % n];

        if y0 == y1 {
            continue;
        }
        if y0 > y1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        if y >= y0 && y < y1 {
            let t = (y - y0) as f64 / (y1 - y0) as f64;
            let x = x0 as f64 + t * (x1 - x0) as f64;
            intersections.push(x);
        }
    }

    intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    intersections
}

fn subtract_span(span: (f64, f64), hole: (f64, f64)) -> Vec<(f64, f64)> {
    let (a, b) = span;
    let (c, d) = hole;
    if d <= a || c >= b {
        return vec![span];
    }
    let mut pieces = Vec::new();
    if c > a {
        pieces.push((a, c));
    }
    if d < b {
        pieces.push((d, b));
    }
    pieces
}

pub fn scanline_fill(img: &mut RgbImage, points: &[(i32, i32)], color: Rgb<u8>, holes: &[&[(i32, i32)]]) {
    let n = points.len();
    if n < 3 {
        return;
    }

    let height = img.dimensions().1;
    let y_min = points.iter().map(|p| p.1).min().unwrap().max(0);
    let y_max = points.iter().map(|p| p.1).max().unwrap().min(height as i32 - 1);

    for y in y_min..=y_max {
        let intersections = row_intersections(points, y);

        let mut spans: Vec<(f64, f64)> = Vec::new();
        let mut i = 0;
        while i + 1 < intersections.len() {
            spans.push((intersections[i], intersections[i + 1]));
            i += 2;
        }

        for hole in holes {
            let hole_intersections = row_intersections(hole, y);
            let mut hole_spans: Vec<(f64, f64)> = Vec::new();
            let mut j = 0;
            while j + 1 < hole_intersections.len() {
                hole_spans.push((hole_intersections[j], hole_intersections[j + 1]));
                j += 2;
            }
            for hole_span in &hole_spans {
                spans = spans
                    .into_iter()
                    .flat_map(|s| subtract_span(s, *hole_span))
                    .collect();
            }
        }

        for (start, end) in spans {
            let x_start = start.round() as i32;
            let x_end = end.round() as i32;
            for x in x_start..=x_end {
                set_pixel(img, x, y, color);
            }
        }
    }
}
