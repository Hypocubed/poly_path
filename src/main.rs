use crate::paths::PolyPath;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod paths;
mod render;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the size of the polygon to use.");
        return;
    }
    let n: i32 = args[1].parse::<i32>().unwrap();

    // Generate all of the path objects.
    let mut paths: Vec<PolyPath> = paths::find_paths(n);
    paths.sort();

    // Print the number of paths.
    println!(
        "Detected {} distinct paths through {} vertices.",
        paths.len(),
        n
    );

    // Create an SVG file's text.
    let text: String = render::svg_from_paths(paths, n);

    let filename = format!("output{}.svg", n);
    let filepath = Path::new(&filename);
    let display = filepath.display();

    // Open a file.
    let mut file = match File::create(&filepath) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the contents string to the file.
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
