FROM ubuntu:latest

#ENV http_proxy http://192.168.174.1:10811
#ENV https_proxy https://192.168.174.1:10811
#ENV no_proxy="127.0.0.1,localhost"
#code server 版本4.4.0

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
RUN wget https://github.com/coder/code-server/releases/download/v4.4.0/code-server-4.4.0-linux-arm64.tar.gz
RUN tar -xzf code-server-4.4.0-linux-arm64.tar.gz


# 安装rust插件包
WORKDIR /root/
RUN wget https://open-vsx.org/api/pinage404/rust-extension-pack/0.1.0/file/pinage404.rust-extension-pack-0.1.0.vsix
RUN /usr/local/code-server-4.4.0-linux-arm64/bin/code-server --install-extension /root/pinage404.rust-extension-pack-0.1.0.vsix

RUN wget https://open-vsx.org/api/vadimcn/vscode-lldb/1.7.0/file/vadimcn.vscode-lldb-1.7.0.vsix
RUN /usr/local/code-server-4.4.0-linux-arm64/bin/code-server --install-extension /root/vadimcn.vscode-lldb-1.7.0.vsix


#暴露端口
EXPOSE 8080

ENTRYPOINT /usr/local/code-server-4.4.0-linux-arm64/bin/code-server

