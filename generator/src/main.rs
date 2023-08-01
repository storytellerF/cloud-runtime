use std::fs::File;
use std::io::prelude::*;

mod android;
mod c;
mod go;
mod java;
mod python;
mod rust;
mod tang;
mod versions;

fn main() {
    let mut file = File::open("../versions.toml").expect("open failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read file failed");
    let config: versions::Config = toml::from_str(&contents).expect("parse failed");

    tang::write(&config);
    c::write(&config);
    rust::write(&config);
    android::write(&config);
    python::write(&config);
    go::write(&config);
    java::write(&config);
    println!("data written to file");
}
