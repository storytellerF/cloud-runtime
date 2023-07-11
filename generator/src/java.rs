
use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path ="common.rs"]
mod common;

pub fn write() {
    write_gradle();
    write_maven();
}

pub fn write_gradle() {
    let mut file = common::file_instance("../java-runtime/code-server-based/gradle.Dockerfile");
    file.write_all(common::ubuntu(vec!["unzip"])).expect("write failed");

    file.write_all(common::manual_install_jdk()).expect("write failed");
    file.write_all(common::manual_install_gradle()).expect("write failed");

    file.write_all(coder::setup_coder(vec![coder::Plugin {
        plugin_key: "vscjava",
        author_name: "vscjava",
        plugin_name: "vscode-java-pack",
        plugin_version: "0.22.4"
    }, coder::Plugin {
        plugin_key: "redhat_analytics",
        author_name: "redhat",
        plugin_name: "fabric8-analytics",
        plugin_version: "0.3.5"
    }, coder::Plugin {
        plugin_key: "redhat_xml",
        author_name: "redhat",
        plugin_name: "vscode-xml",
        plugin_version: "0.20.0"
    }]).as_bytes()).expect("write failed");
}

pub fn write_maven() {
    let mut file = common::file_instance("../java-runtime/code-server-based/maven.Dockerfile");
    file.write_all(common::ubuntu(vec!["unzip"])).expect("write failed");

    file.write_all(common::manual_install_jdk()).expect("write failed");
    file.write_all(common::manual_install_maven()).expect("write failed");

    file.write_all(coder::setup_coder(vec![coder::Plugin {
        plugin_key: "vscjava",
        author_name: "vscjava",
        plugin_name: "vscode-java-pack",
        plugin_version: "0.22.4"
    }, coder::Plugin {
        plugin_key: "redhat_analytics",
        author_name: "redhat",
        plugin_name: "fabric8-analytics",
        plugin_version: "0.3.5"
    }, coder::Plugin {
        plugin_key: "redhat_xml",
        author_name: "redhat",
        plugin_name: "vscode-xml",
        plugin_version: "0.20.0"
    }]).as_bytes()).expect("write failed");
}