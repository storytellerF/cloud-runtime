use std::io::Write;

use crate::versions;
use crate::coder;
use crate::common;

pub fn write(config: &versions::Config) {
    write_dockerfile(config, true, "code-server-based");
    write_dockerfile(config, false, "default");
}

fn write_dockerfile(config: &versions::Config, add_coder_server: bool, dir: &str) {
    let mut file = common::file_instance(format!("../rust-runtime/{}/Dockerfile", dir).as_str());
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");
    file.write_all(
        "
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    RUN . ~/.cargo/env
    "
        .as_bytes(),
    )
    .expect("write failed");

    if add_coder_server {
        file.write_all(
            coder::setup_coder(
                vec![coder::Plugin {
                    plugin_key: "extension_pack",
                    author_name: "pinage404",
                    plugin_name: "rust-extension-pack",
                    plugin_version: &config.versions.rust_pack,
                }],
                config,
            )
            .as_bytes(),
        )
        .expect("write failed");
    }
}
