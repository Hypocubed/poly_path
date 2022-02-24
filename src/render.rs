use crate::paths::PolyPath;
use num::complex::Complex;
use std::f64::consts::TAU;

const SCALE: i32 = 50;

pub fn get_polygon_verticies(n: i32) -> Vec<Complex<f64>> {
    let mut result: Vec<Complex<f64>> = vec![];

    for x in 0..n {
        result.push(Complex::new(
            (x as f64 * TAU / n as f64).sin(),
            -(x as f64 * TAU / n as f64).cos(),
        ));
    }

    result
}

pub fn svg_shape_from_path(path: &PolyPath, x: i32, y: i32) -> String {
    let n: i32 = path.size as i32;

    let verts: Vec<Complex<f64>> = get_polygon_verticies(n as i32);
    let mut points: Vec<Complex<i32>> = vec![];

    for vert in verts.iter() {
        points.push(Complex::new(
            ((vert.re * SCALE as f64).round() as i32) + x + (2 * SCALE),
            ((vert.im * SCALE as f64).round() as i32) + y + (2 * SCALE),
        ));
    }

    let mut result: String = String::from("");

    // Print out the background.
    result.push_str(
        format!(
            "<rect fill=\"#fff\" stroke=\"#000\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
            x,
            y,
            (4 * SCALE),
            (4 * SCALE)
        )
        .as_str(),
    );

    // Print out the label.
    let mut label: String = String::from("");
    for jump in path.path.iter() {
        label.push_str(&jump.to_string());
    }

    result.push_str(format!("<text x=\"{}\" y=\"{}\">{}</text>", x + 10, y + 15, label).as_str());

    // Print out the poly line.
    let mut current: i32 = 0;
    result.push_str("<polyline points=\"");
    result.push_str(format!("{},{}", points[0].re, points[0].im).as_str());
    for x in path.path.iter() {
        current = (current + x) % n;

        result.push_str(
            format!(
                " {},{}",
                points[current as usize].re, points[current as usize].im
            )
            .as_str(),
        );
    }
    result.push_str("\" stroke=\"black\" stroke-width=\"3\" fill=\"none\" />\r\n");

    // Print out verts
    for i in 0..n {
        let point = points[i as usize];

        result.push_str(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"red\" />\r\n",
                point.re, point.im
            )
            .as_str(),
        );
    }
    let start = points[0];
    result.push_str(
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"white\" />\r\n",
            start.re, start.im
        )
        .as_str(),
    );

    result
}

pub fn svg_from_path(path: &PolyPath) -> String {
    let mut result: String = String::from("");

    // Print the header
    result.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\r\n");
    result.push_str("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\r\n");
    result.push_str("<svg width=\"200\" height=\"200\" viewBox=\"-100 -100 200 200\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\r\n");

    // Print out the path.
    svg_shape_from_path(path, 0, 0);

    // End the file.
    result.push_str("</svg>");

    result
}

pub fn svg_from_paths(paths: Vec<PolyPath>) -> String {
    let mut result: String = String::from("");
    let count: i32 = paths.len() as i32;

    let square = (count as f64).sqrt().ceil() as i32;

    // Print the header
    result.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\r\n");
    result.push_str("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\r\n");
    result.push_str(
        format!("<svg width=\"{0}\" height=\"{0}\" viewBox=\"0 0 {0} {0}\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\r\n",
        4 * SCALE * square
    ).as_str());
    result.push_str(
        format!(
            "<rect fill=\"#999\" stroke=\"#000\" x=\"0\" y=\"0\" width=\"{0}\" height=\"{0}\"/>",
            4 * SCALE * square
        )
        .as_str(),
    );

    // Print out the paths.
    let mut count: i32 = 0;
    let mut x: i32;
    let mut y: i32;

    for path in paths.iter() {
        x = count % square;
        y = count / square;

        let shape = svg_shape_from_path(path, 4 * x * SCALE, 4 * y * SCALE);
        result.push_str(&shape);
        count = count + 1;
    }

    // End the file.
    result.push_str("</svg>");

    result
}
