FROM golang:1.16

ENV http_proxy http://192.168.20.1:10809
ENV https_proxy http://192.168.20.1:10809

ARG cs_version=3.11.0
ADD https://github.com/cdr/code-server/releases/download/v$cs_version/code-server-$cs_version-linux-amd64.tar.gz /usr/local/code-server/
# 解压
WORKDIR /usr/local/code-server
RUN tar -xzf code-server-$cs_version-linux-amd64.tar.gz

# ADD code-server-$cs_version-linux-amd64.tar.gz /usr/local/code-server/
ENV code_server /usr/local/code-server/code-server-$cs_version-linux-amd64

# https://github.com/cdr/code-server/releases/tag/v$cs_version

# 下载插件

# ADD https://marketplace.visualstudio.com/_apis/public/gallery/publishers/golang/vsextensions/Go/0.26.0/vspackage /root/
# RUN ${code_server}/bin/code-server --install-extension /root/golang.Go-0.26.0.vsix
#formulahendry.code-runner.0.11.5.vsix
ADD https://marketplace.visualstudio.com/_apis/public/gallery/publishers/formulahendry/vsextensions/code-runner/0.11.5/vspackage /root/formulahendry.code-runner-0.11.5.vsix
RUN ${code_server}/bin/code-server --install-extension /root/formulahendry.code-runner-0.11.5.vsix

ADD https://marketplace.visualstudio.com/_apis/public/gallery/publishers/ms-python/vsextensions/python/2021.7.1060902895/vspackage /root/ms-python.python-2021.7.1060902895.vsix
RUN ${code_server}/bin/code-server --install-extension /root/ms-python.python-2021.7.1060902895.vsix

# 安装git
RUN apt-get update
RUN apt-get upgrade
RUN apt-get -y install git
RUN apt-get install python3
RUN pip install --upgrade pip
RUN pip install tensorflow
RUN pip install tf-nightly


# 启动服务器

EXPOSE 8080

ENTRYPOINT ${code_server}/bin/code-server --host 0.0.0.0 --port 8080

