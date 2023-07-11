FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git vim build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . ~/.cargo/env


ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN curl -O https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz

#安装extension-pack
WORKDIR /root/
ARG extension_pack_version=0.1.0
ARG extension_pack_author=pinage404
ARG extension_pack_artifact=rust-extension-pack
ARG extension_pack_name=$extension_pack_author.$extension_pack_artifact-${extension_pack_version}.vsix
RUN curl -O https://open-vsx.org/api/$extension_pack_author/$extension_pack_artifact/${extension_pack_version}/file/$extension_pack_name
RUN ${code_server_executable} --install-extension /root/$extension_pack_name
#安装extension-pack结束


RUN touch /start.sh
RUN echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh && chmod +x /start.sh
ENTRYPOINT /start.sh