mod android;
mod c;
mod go;
mod java;
mod python;
mod rust;
mod tang;

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
