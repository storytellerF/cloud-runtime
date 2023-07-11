
use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path ="common.rs"]
mod common;

pub fn write() {
    let mut file = common::fileInstance("../rust-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["build-essential"])).expect("write failed");
    file.write_all(
        "
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN source ~/.cargo/env
\n"
            .as_bytes(),
    )
    .expect("write failed");

    coder::setup_coder(file, vec![coder::Plugin {
        plugin_key: "extension-pack",
        author_name: "pinage404",
        plugin_name: "rust-extension-pack",
        plugin_version: "0.1.0"
    }]);
}