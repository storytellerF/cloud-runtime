use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

use crate::versions;

pub fn write(config: &versions::Config) {
    let mut file = common::file_instance("../tang-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");
    file.write_all(
        "
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.cargo/env
RUN . $HOME/.cargo/env \\
    && rustup override add nightly \\
    && rustup component add rust-src \\
    && cargo install bootimage \\
    && rustup component add llvm-tools-preview\n"
            .as_bytes(),
    )
    .expect("write failed");
    file.write_all(coder::setup_coder(vec![], config).as_bytes())
        .expect("write failed");
}
