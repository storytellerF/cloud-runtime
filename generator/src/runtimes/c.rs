use std::io::Write;

use crate::versions;
use crate::coder;
use crate::common;

pub fn write(config: &versions::Config) {
    write_dockerfile(config, true, "code-server-based");
    write_dockerfile(config, false, "default");
}

fn write_dockerfile(config: &versions::Config, add_coder_server: bool, dir: &str) {
    let mut file = common::file_instance(format!("../c-runtime/{}/Dockerfile", dir).as_str());
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");

    if add_coder_server {
        file.write_all(
            coder::setup_coder(
                vec![coder::Plugin {
                    plugin_key: "code_runner",
                    author_name: "formulahendry",
                    plugin_name: "code-runner",
                    plugin_version: &config.versions.code_runner,
                }],
                config,
            )
            .as_bytes(),
        )
        .expect("write failed");
    }
}
