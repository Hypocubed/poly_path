use crate::paths::PolyPath;
use std::char;
use std::f64::consts::TAU;
use std::slice;

const SCALE: i32 = 50;

pub fn get_polygon_vertices(n: i32) -> Vec<(f64, f64)> {
    let mut result = vec![];

    for x in 0..n {
        let angle = x as f64 * TAU / n as f64;
        result.push((angle.sin(), -angle.cos()));
    }

    result
}

pub fn point_from_vertex(vert: (f64, f64), x: i32, y: i32) -> (i32, i32) {
    (
        (vert.0 * SCALE as f64).round() as i32 + 4 * SCALE * x + 2 * SCALE,
        (vert.1 * SCALE as f64).round() as i32 + 4 * SCALE * y + 2 * SCALE,
    )
}

pub fn svg_labels_from_paths(paths: &[PolyPath], square: i32) -> String {
    let mut result = String::from("");

    for (i, path) in paths.iter().enumerate() {
        let (x, y) = (i as i32 % square, i as i32 / square);
        let label: String = path
            .path
            .iter()
            .map(|&x| char::from_digit(x as u32, 36).unwrap().to_ascii_uppercase())
            .collect();
        result.push_str(&format!(
            "<text x=\"{}\" y=\"{}\">{}</text>\r\n",
            4 * SCALE * x + 10,
            4 * SCALE * y + 15,
            label
        ));
    }

    result
}

pub fn svg_shapes_from_paths(paths: &[PolyPath], square: i32, n: i32) -> String {
    let mut result = String::from("");
    let verts = get_polygon_vertices(n);

    result.push_str(
        "<g stroke=\"black\" stroke-linejoin=\"bevel\" stroke-width=\"3\" fill=\"none\" \
            marker-start=\"url(#top-vert)\" marker-mid=\"url(#vert)\">\r\n",
    );
    for (i, path) in paths.iter().enumerate() {
        let (x, y) = (i as i32 % square, i as i32 / square);
        let mut points = String::from("");
        let mut current = 0;

        for jump in &path.path {
            let point = point_from_vertex(verts[current as usize], x, y);
            current = (current + jump) % n;
            points.push_str(&format!("L{},{}", point.0, point.1));
        }

        result.push_str(&format!("<path d=\"M{}Z\"/>\r\n", &points[1..]));
    }
    result.push_str("</g>\r\n");

    result
}

pub fn svg_from_path(path: &PolyPath) -> String {
    let mut result = String::from("");

    // Print out the header and background.
    result.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\r\n\
        <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
            \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\r\n\
        <svg width=\"{0}\" height=\"{0}\" viewBox=\"0 0 {0} {0}\" \
            xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\r\n\
        <defs>\r\n\
        <marker id=\"top-vert\" viewBox=\"-5 -5 10 10\" markerUnits=\"userSpaceOnUse\" \
            markerWidth=\"10\" markerHeight=\"10\"><circle r=\"5\" fill=\"red\"/><circle r=\"3\" \
            fill=\"white\"/></marker>\r\n\
        <marker id=\"vert\" viewBox=\"-5 -5 10 10\" markerUnits=\"userSpaceOnUse\" \
            markerWidth=\"10\" markerHeight=\"10\"><circle r=\"5\" fill=\"red\"/></marker>\r\n\
        </defs>\r\n\
        <rect x=\".5\" y=\".5\" width=\"{1}\" height=\"{1}\" stroke=\"black\" fill=\"white\"/>\r\n",
        4 * SCALE + 1,
        4 * SCALE
    ));

    // Print out the label and path.
    result.push_str(&svg_labels_from_paths(slice::from_ref(path), 1));
    result.push_str(&svg_shapes_from_paths(slice::from_ref(path), 1, path.size));

    // End the file.
    result.push_str("</svg>");

    result
}

pub fn svg_from_paths(paths: Vec<PolyPath>, n: i32) -> String {
    let mut result = String::from("");
    let count = paths.len() as i32;
    let square = (count as f64).sqrt().ceil() as i32;

    // Print out the header and background.
    result.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\r\n\
        <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
            \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\r\n\
        <svg width=\"{0}\" height=\"{0}\" viewBox=\"0 0 {0} {0}\" \
            xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\r\n\
        <defs>\r\n\
        <pattern id=\"grid\" patternUnits=\"userSpaceOnUse\" viewBox=\"0 0 {1} {1}\" width=\"{1}\" \
            height=\"{1}\"><rect x=\".5\" y=\".5\" width=\"{1}\" height=\"{1}\" stroke=\"black\" \
            fill=\"white\" /></pattern>\r\n\
        <marker id=\"top-vert\" viewBox=\"-5 -5 10 10\" markerUnits=\"userSpaceOnUse\" \
            markerWidth=\"10\" markerHeight=\"10\"><circle r=\"5\" fill=\"red\"/><circle r=\"3\" \
            fill=\"white\"/></marker>\r\n\
        <marker id=\"vert\" viewBox=\"-5 -5 10 10\" markerUnits=\"userSpaceOnUse\" \
            markerWidth=\"10\" markerHeight=\"10\"><circle r=\"5\" fill=\"red\"/></marker>\r\n\
        </defs>\r\n\
        <rect x=\".5\" y=\".5\" width=\"{2}\" height=\"{2}\" stroke=\"black\" fill=\"#999\"/>\r\n\
        <rect width=\"{0}\" height=\"{3}\" fill=\"url(#grid)\"/>\r\n",
        4 * SCALE * square + 1,
        4 * SCALE,
        4 * SCALE * square,
        4 * SCALE * (count / square) + 1
    ));

    // Print out a partial row if necessary.
    if count % square != 0 {
        result.push_str(&format!(
            "<rect y=\"{}\" width=\"{}\" height=\"{}\" fill=\"url(#grid)\"/>\r\n",
            4 * SCALE * (count / square),
            4 * SCALE * (count % square) + 1,
            4 * SCALE + 1
        ));
    }

    // Print out the labels and paths.
    result.push_str(&svg_labels_from_paths(&paths, square));
    result.push_str(&svg_shapes_from_paths(&paths, square, n));

    // End the file.
    result.push_str("</svg>");

    result
}
