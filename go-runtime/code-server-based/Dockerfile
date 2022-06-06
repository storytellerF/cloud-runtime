FROM ubuntu:20.04 
#与default 中的不同，default中的直接使用go-lang的镜像。

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install wget curl -y

ENV code_server_version=4.4.0
ENV code_server_flavor=${code_server_version}-linux-arm64
ENV code_server_executable=/usr/local/code-server-${code_server_flavor}/bin/code-server
ENV go_version=0.33.1

WORKDIR /usr/local/
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/code-server-${code_server_flavor}.tar.gz
RUN tar -xzf code-server-${code_server_flavor}.tar.gz

# ADD golang.Go-0.26.0.vsix /root/
WORKDIR /root/
RUN wget https://open-vsx.org/api/golang/Go/${go_version}/file/golang.Go-${go_version}.vsix
RUN ${code_server_executable} --install-extension /root/golang.Go-${go_version}.vsix

EXPOSE 8080

ENTRYPOINT ${code_server_executable}