
use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path ="common.rs"]
mod common;

pub fn write() {
    let mut file = common::fileInstance("../tang-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["build-essential"])).expect("write failed");
    file.write_all(
        "
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN source ~/.cargo/env
RUN rustup override add nightly
RUN rustup component add rust-src
RUN cargo install bootimage
RUN rustup component add llvm-tools-preview\n"
            .as_bytes(),
    )
    .expect("write failed");
    coder::setup_coder(file, vec![]);
}