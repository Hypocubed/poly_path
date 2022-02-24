use crate::paths::PolyPath;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod paths;
mod render;

fn main() {
    // Generate all of the path objects.
    let mut paths: Vec<PolyPath> = paths::find_paths(6);
    paths.sort();

    // Create an SVG file's text.
    let text: String = render::svg_from_paths(paths);

    let path = Path::new("output6.svg");
    let display = path.display();

    // Open a file.
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the contents string to the file.
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
