FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git vim build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN source ~/.cargo/env


ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz

#安装extension-pack
WORKDIR /root/
ARG extension-pack_version=0.1.0
ARG extension-pack_author=pinage404
ARG extension-pack_artifact=rust-extension-pack
ARG extension-pack_name=$extension-pack_author.$extension-pack_artifact-${extension-pack_version}.vsix
RUN wget https://open-vsx.org/api/$extension-pack_author/$extension-pack_artifact/${extension-pack_version}/file/$extension-pack_name
RUN ${code_server_executable} --install-extension /root/$extension-pack_name
#安装extension-pack结束


COPY start.sh /
RUN chmod +x /start.sh && echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh
ENTRYPOINT /start.sh