FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git vim 
ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz

#安装go_lang
WORKDIR /root/
ARG go_lang_version=0.26.0
ARG go_lang_author=golang
ARG go_lang_artifact=Go
ARG go_lang_name=$go_lang_author.$go_lang_artifact-${go_lang_version}.vsix
RUN wget https://open-vsx.org/api/$go_lang_author/$go_lang_artifact/${go_lang_version}/file/$go_lang_name
RUN ${code_server_executable} --install-extension /root/$go_lang_name
#安装go_lang结束


COPY start.sh /
RUN chmod +x /start.sh && echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh
ENTRYPOINT /start.sh