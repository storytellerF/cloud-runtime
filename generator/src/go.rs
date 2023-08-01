use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

use crate::versions;

pub fn write(config: &versions::Config) {
    let mut file = common::file_instance("../go-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec![]))
        .expect("write failed");

    file.write_all(
        coder::setup_coder(vec![coder::Plugin {
            plugin_key: "go_lang",
            author_name: "golang",
            plugin_name: "Go",
            plugin_version: &config.versions.go_lang,
        }], config)
        .as_bytes(),
    )
    .expect("write failed");
}
