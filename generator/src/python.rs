use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

pub fn write() {
    let mut file =
        common::fileInstance("../python-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["python3", "python3-pip"]))
        .expect("write failed");

    coder::setup_coder(
        file,
        vec![coder::Plugin {
            plugin_key: "code_runner",
            author_name: "formulahendry",
            plugin_name: "code-runner",
            plugin_version: "0.11.7",
        }, coder::Plugin {
            plugin_key: "python_plugin",
            author_name: "ms-python",
            plugin_name: "python",
            plugin_version: "2022.6.3"
        }],
    );
}
