mod paths;
mod render;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut paths: Vec<Vec<i32>> = paths::find_paths(7);
    paths.sort();

    let text: String = render::svg_from_paths(paths);

    let path = Path::new("output7.svg");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
