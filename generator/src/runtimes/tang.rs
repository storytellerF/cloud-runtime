use std::io::Write;

use crate::versions;
use crate::coder;
use crate::common;

pub fn write(config: &versions::Config) {
    write_dockerfile(config, true, "code-server-based");
    write_dockerfile(config, false, "default");
}

fn write_dockerfile(config: &versions::Config, add_coder_server: bool, dir: &str) {
    let mut file = common::file_instance(format!("../tang-runtime/{}/Dockerfile", dir).as_str());
    file.write_all(common::ubuntu(vec!["build-essential"]))
        .expect("write failed");
    file.write_all(
        "
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.cargo/env
RUN . $HOME/.cargo/env \\
    && rustup override add nightly \\
    && rustup component add rust-src \\
    && cargo install bootimage \\
    && rustup component add llvm-tools-preview\n"
            .as_bytes(),
    )
    .expect("write failed");
    if add_coder_server {
        file.write_all(coder::setup_coder(vec![], config).as_bytes())
            .expect("write failed");
    }
}
