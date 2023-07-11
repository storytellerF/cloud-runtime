mod tang;
mod coder;
mod common;
mod c;
mod rust;
mod android;
mod python;
mod go;
mod java;

fn main() {
    tang::write();
    c::write();
    rust::write();
    android::write();
    python::write();
    go::write();
    java::write();
    println!("data written to file");
}


