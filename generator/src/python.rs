use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

use crate::versions;


pub fn write(config: &versions::Config) {
    let mut file = common::file_instance("../python-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["python3", "python3-pip"]))
        .expect("write failed");

    file.write_all(
        coder::setup_coder(vec![
            coder::Plugin {
                plugin_key: "code_runner",
                author_name: "formulahendry",
                plugin_name: "code-runner",
                plugin_version: &config.versions.code_runner,
            },
            coder::Plugin {
                plugin_key: "python_plugin",
                author_name: "ms-python",
                plugin_name: "python",
                plugin_version: &config.versions.vsc_python,
            },
        ], config)
        .as_bytes(),
    )
    .expect("write failed");
}
