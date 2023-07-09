FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get curl install -y git vim build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN source ~/.cargo/env
RUN rustup override add nightly
RUN rustup component add rust-src
RUN cargo install bootimage
RUN rustup component add llvm-tools-preview
ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz
COPY start.sh /
RUN chmod +x /start.sh && echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh
ENTRYPOINT /start.sh