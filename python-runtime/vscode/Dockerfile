FROM ubuntu:latest

# ENV http_proxy http://192.168.20.1:10809
# ENV https_proxy http://192.168.20.1:10809

ENV code_server_version=4.4.0
ENV code_server_flavor=${code_server_version}-linux-arm64
ENV code_server_executable=/usr/local/code-server-${code_server_flavor}/bin/code-server

# 安装git
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install git wget -y
RUN apt-get install python3 -y
RUN apt-get install python3-pip -y
RUN pip3 install --upgrade pip

WORKDIR /usr/local/
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/code-server-${code_server_flavor}.tar.gz
RUN tar -xzf code-server-${code_server_flavor}.tar.gz

# 下载插件
WORKDIR /root/
ARG code_runner_version=0.11.7
RUN wget https://open-vsx.org/api/formulahendry/code-runner/${code_runner_version}/file/formulahendry.code-runner-${code_runner_version}.vsix
RUN ${code_server_executable} --install-extension /root/formulahendry.code-runner-${code_runner_version}.vsix

ARG python_extension_pkg_version=2022.6.3
RUN wget https://open-vsx.org/api/ms-python/python/${python_extension_pkg_version}/file/ms-python.python-${python_extension_pkg_version}.vsix
RUN ${code_server_executable} --install-extension /root/ms-python.python-${python_extension_pkg_version}.vsix

# 启动服务器

EXPOSE 8080

ENTRYPOINT ${code_server_executable}/bin/code-server --host 0.0.0.0 --port 8080

