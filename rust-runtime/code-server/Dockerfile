FROM ubuntu:latest

#ENV http_proxy http://192.168.174.1:10811
#ENV https_proxy https://192.168.174.1:10811
#ENV no_proxy="127.0.0.1,localhost"
#code server 版本4.4.0

ENV code_server_version=4.4.0
ENV code_server_flavor=${code_server_version}-linux-arm64
ENV code_server_executable=/usr/local/code-server-${code_server_flavor}/bin/code-server

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install wget curl -y

RUN apt-get install build-essential -y

RUN curl -o rustup.sh --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
RUN chmod 777 ./rustup.sh
RUN ./rustup.sh -y
RUN ~/.cargo/bin/rustup component add rust-analysis --toolchain stable-aarch64-unknown-linux-gnu
RUN ~/.cargo/bin/rustup component add rust-src --toolchain stable-aarch64-unknown-linux-gnu
RUN ~/.cargo/bin/rustup component add rls --toolchain stable-aarch64-unknown-linux-gnu



#code server
WORKDIR /usr/local/
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/code-server-${code_server_flavor}.tar.gz
RUN tar -xzf code-server-${code_server_flavor}.tar.gz


# 安装rust插件包
WORKDIR /root/
ENV rust_extension_version=0.1.0
ARG rust_extension_name=pinage404.rust-extension-pack-${rust_extension_version}.vsix
RUN wget https://open-vsx.org/api/pinage404/rust-extension-pack/${rust_extension_version}/file/${rust_extension_name}
RUN ${code_server_executable} --install-extension /root/${rust_extension_name}

ENV lldb_extension_version=1.7.0
ENV lldb_extension_name=vadimcn.vscode-lldb-${lldb_extension_version}.vsix
RUN wget https://open-vsx.org/api/vadimcn/vscode-lldb/${lldb_extension_version}/file/${lldb_extension_name}
RUN ${code_server_executable} --install-extension /root/${lldb_extension_name}


#暴露端口
EXPOSE 8080

ENTRYPOINT ${code_server_executable}

