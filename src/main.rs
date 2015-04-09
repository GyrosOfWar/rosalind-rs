use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

mod profile_matrix;
mod genome;

fn main() {
    let table = HashMap::new();
    genome::open_reading_frames("AUGAGUAAG", table);
}
