use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

pub fn write() {
    let mut file =
        common::fileInstance("../c-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");

    coder::setup_coder(
        file,
        vec![coder::Plugin {
            plugin_key: "code_runner",
            author_name: "formulahendry",
            plugin_name: "code-runner",
            plugin_version: "0.11.7",
        }],
    );
}
