use std::fs::File;
use std::io::prelude::*;

use generator::{build, versions};

fn main() {
    let mut file = File::open("../versions.toml").expect("open failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("read file failed");
    let config: versions::Config = toml::from_str(&contents).expect("parse failed");

    build(&config);
    println!("data written to file");
}
