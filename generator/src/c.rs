use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

pub fn write() {
    let mut file = common::file_instance("../c-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");

    file.write_all(
        coder::setup_coder(vec![coder::Plugin {
            plugin_key: "code_runner",
            author_name: "formulahendry",
            plugin_name: "code-runner",
            plugin_version: "0.11.7",
        }])
        .as_bytes(),
    )
    .expect("write failed");
}
