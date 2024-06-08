mod coder;
mod common;
pub mod versions;
pub mod runtimes;

use crate::runtimes::*;

pub fn build(config: &versions::Config) {
    android::write(&config);
    tang::write(&config);
    c::write(&config);
    rust::write(&config);
    android::write(&config);
    python::write(&config);
    go::write(&config);
    java::write(&config);
}