mod tang;
mod coder;
mod common;
mod c;
mod rust;
mod android;
mod python;
mod go;

fn main() {
    tang::write();
    c::write();
    rust::write();
    android::write();
    python::write();
    go::write();
    println!("data written to file");
}


