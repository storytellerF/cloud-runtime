use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;
//cargo clippy rust-docs rust-std rustc

pub fn write() {
    let mut file = common::file_instance("../rust-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");
    file.write_all(
        "
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . ~/.cargo/env
\n"
        .as_bytes(),
    )
    .expect("write failed");

    file.write_all(
        coder::setup_coder(vec![coder::Plugin {
            plugin_key: "extension_pack",
            author_name: "pinage404",
            plugin_name: "rust-extension-pack",
            plugin_version: "0.1.0",
        }])
        .as_bytes(),
    )
    .expect("write failed");
}
