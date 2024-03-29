FROM ubuntu:latest
RUN apt-get update && apt-get -y install ca-certificates
ARG sourcesRaw=sources.list.prefer
COPY ./${sourcesRaw}.a /etc/apt/${sourcesRaw}.a
COPY ./${sourcesRaw}.x /etc/apt/${sourcesRaw}.x

WORKDIR /etc/apt
RUN mv sources.list sources.list.backup \
        && cp ${sourcesRaw}.$(uname -m | cut -c1) sources.list \
        && apt-get update && apt-get upgrade -y && apt-get install -y curl git vim \
        && apt-get install -y python3 python3-pip
ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_pack_name=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_sub_path=bin/code-server
#ARG code_server_executable=${code_server_parent}/${code_server_pack_name}/${code_server_sub_path}
ARG code_server_executable=/usr/${code_server_sub_path}

# 使用官方脚本安装更好，旧版本的安装方式禁用
#code server
#WORKDIR ${code_server_parent}
#RUN curl -LO https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_pack_name}.tar.gz
#RUN tar -xzf ${code_server_pack_name}.tar.gz

RUN curl -fsSL https://code-server.dev/install.sh | sh 

#安装git_lens
WORKDIR /root/
ARG git_lens_version=14.0.1
ARG git_lens_author=eamodio
ARG git_lens_artifact=gitlens
ARG git_lens_name=$git_lens_author.$git_lens_artifact-${git_lens_version}.vsix
RUN curl -LO https://open-vsx.org/api/$git_lens_author/$git_lens_artifact/${git_lens_version}/file/$git_lens_name \
    && ${code_server_executable} --install-extension /root/$git_lens_name \
    && rm /root/$git_lens_name
#安装git_lens结束

#安装git_ignore_plugin
WORKDIR /root/
ARG git_ignore_plugin_version=0.9.0
ARG git_ignore_plugin_author=codezombiech
ARG git_ignore_plugin_artifact=gitignore
ARG git_ignore_plugin_name=$git_ignore_plugin_author.$git_ignore_plugin_artifact-${git_ignore_plugin_version}.vsix
RUN curl -LO https://open-vsx.org/api/$git_ignore_plugin_author/$git_ignore_plugin_artifact/${git_ignore_plugin_version}/file/$git_ignore_plugin_name \
    && ${code_server_executable} --install-extension /root/$git_ignore_plugin_name \
    && rm /root/$git_ignore_plugin_name
#安装git_ignore_plugin结束

#安装code_runner
WORKDIR /root/
ARG code_runner_version=0.12.0
ARG code_runner_author=formulahendry
ARG code_runner_artifact=code-runner
ARG code_runner_name=$code_runner_author.$code_runner_artifact-${code_runner_version}.vsix
RUN curl -LO https://open-vsx.org/api/$code_runner_author/$code_runner_artifact/${code_runner_version}/file/$code_runner_name \
    && ${code_server_executable} --install-extension /root/$code_runner_name \
    && rm /root/$code_runner_name
#安装code_runner结束

#安装python_plugin
WORKDIR /root/
ARG python_plugin_version=2022.6.3
ARG python_plugin_author=ms-python
ARG python_plugin_artifact=python
ARG python_plugin_name=$python_plugin_author.$python_plugin_artifact-${python_plugin_version}.vsix
RUN curl -LO https://open-vsx.org/api/$python_plugin_author/$python_plugin_artifact/${python_plugin_version}/file/$python_plugin_name \
    && ${code_server_executable} --install-extension /root/$python_plugin_name \
    && rm /root/$python_plugin_name
#安装python_plugin结束


RUN echo $code_server_executable --bind-addr 0.0.0.0:8080 > /start.sh && chmod +x /start.sh
ENTRYPOINT /start.sh