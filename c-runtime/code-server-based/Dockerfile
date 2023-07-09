FROM ubuntu:latest

#code server 版本4.4.0

ENV code_server_version=4.4.0
ENV code_server_flavor=${code_server_version}-linux-arm64
ENV code_server_executable=/usr/local/code-server-${code_server_flavor}/bin/code-server

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install wget curl -y

RUN apt-get install build-essential -y

#code server
WORKDIR /usr/local/
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/code-server-${code_server_flavor}.tar.gz
RUN tar -xzf code-server-${code_server_flavor}.tar.gz

WORKDIR /root/
ARG code_runner_version=0.11.7
RUN wget https://open-vsx.org/api/formulahendry/code-runner/${code_runner_version}/file/formulahendry.code-runner-${code_runner_version}.vsix
RUN ${code_server_executable} --install-extension /root/formulahendry.code-runner-${code_runner_version}.vsix

#暴露端口
EXPOSE 8080

ENTRYPOINT ${code_server_executable}

