FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git vim python3 python3-pip
ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz

#安装code_runner
WORKDIR /root/
ARG code_runner_version=0.11.7
ARG code_runner_author=formulahendry
ARG code_runner_artifact=code-runner
ARG code_runner_name=$code_runner_author.$code_runner_artifact-${code_runner_version}.vsix
RUN wget https://open-vsx.org/api/$code_runner_author/$code_runner_artifact/${code_runner_version}/file/$code_runner_name
RUN ${code_server_executable} --install-extension /root/$code_runner_name
#安装code_runner结束

#安装python_plugin
WORKDIR /root/
ARG python_plugin_version=2022.6.3
ARG python_plugin_author=ms-python
ARG python_plugin_artifact=python
ARG python_plugin_name=$python_plugin_author.$python_plugin_artifact-${python_plugin_version}.vsix
RUN wget https://open-vsx.org/api/$python_plugin_author/$python_plugin_artifact/${python_plugin_version}/file/$python_plugin_name
RUN ${code_server_executable} --install-extension /root/$python_plugin_name
#安装python_plugin结束


COPY start.sh /
RUN chmod +x /start.sh && echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh
ENTRYPOINT /start.sh