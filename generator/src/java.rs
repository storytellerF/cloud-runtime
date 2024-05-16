use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path = "common.rs"]
mod common;

use crate::versions;

pub fn write(config: &versions::Config) {
    write_gradle(config, true, "code-server-based");
    write_gradle(config, false, "default");
    write_maven(config, true, "code-server-based");
    write_maven(config, false, "default");
}

pub fn write_gradle(config: &versions::Config, add_coder_server: bool, dir: &str) {
    let mut file =
        common::file_instance(format!("../java-runtime/{}/gradle.Dockerfile", dir).as_str());
    file.write_all(common::ubuntu(vec!["unzip"]))
        .expect("write failed");

    file.write_all(common::manual_install_jdk())
        .expect("write failed");
    file.write_all(common::manual_install_gradle())
        .expect("write failed");

    if add_coder_server {
        file.write_all(
            coder::setup_coder(
                vec![
                    coder::Plugin {
                        plugin_key: "vscjava",
                        author_name: "vscjava",
                        plugin_name: "vscode-java-pack",
                        plugin_version: &config.versions.java_pack,
                    },
                    coder::Plugin {
                        plugin_key: "redhat_analytics",
                        author_name: "redhat",
                        plugin_name: "fabric8-analytics",
                        plugin_version: &config.versions.redhat_analytics,
                    },
                    coder::Plugin {
                        plugin_key: "redhat_xml",
                        author_name: "redhat",
                        plugin_name: "vscode-xml",
                        plugin_version: &config.versions.redhat_xml,
                    },
                ],
                config,
            )
            .as_bytes(),
        )
        .expect("write failed");
    }
}

pub fn write_maven(config: &versions::Config, add_coder_server: bool, dir: &str) {
    let mut file = common::file_instance(format!("../java-runtime/{}/maven.Dockerfile", dir).as_str());
    file.write_all(common::ubuntu(vec!["unzip"]))
        .expect("write failed");

    file.write_all(common::manual_install_jdk())
        .expect("write failed");
    file.write_all(common::manual_install_maven())
        .expect("write failed");

    if add_coder_server {
        file.write_all(
            coder::setup_coder(
                vec![
                    coder::Plugin {
                        plugin_key: "vscjava",
                        author_name: "vscjava",
                        plugin_name: "vscode-java-pack",
                        plugin_version: "0.22.4",
                    },
                    coder::Plugin {
                        plugin_key: "redhat_analytics",
                        author_name: "redhat",
                        plugin_name: "fabric8-analytics",
                        plugin_version: "0.3.5",
                    },
                    coder::Plugin {
                        plugin_key: "redhat_xml",
                        author_name: "redhat",
                        plugin_name: "vscode-xml",
                        plugin_version: "0.20.0",
                    },
                ],
                config,
            )
            .as_bytes(),
        )
        .expect("write failed");
    }
}
