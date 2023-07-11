use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

pub fn write() {
    let mut file =
        common::fileInstance("../go-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec![]))
        .expect("write failed");

    coder::setup_coder(
        file,
        vec![coder::Plugin {
            plugin_key: "go_lang",
            author_name: "golang",
            plugin_name: "Go",
            plugin_version: "0.26.0",
        }],
    );
}
