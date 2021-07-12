FROM golang:1.16

ENV http_proxy http://192.168.20.1:10809
ENV https_proxy http://192.168.20.1:10809

#ADD https://github.com/cdr/code-server/releases/download/v3.10.2/code-server-3.10.2-linux-amd64.tar.gz /usr/local/code-server/
# 解压
#WORKDIR /usr/local/code-server
#RUN tar -xzf code-server-3.10.2-linux-amd64.tar.gz

# 这个需要事先下载好，上面会重新下载
ADD code-server-3.10.2-linux-amd64.tar.gz /usr/local/code-server/
ENV code_server /usr/local/code-server/code-server-3.10.2-linux-amd64
# https://github.com/cdr/code-server/releases/tag/v3.10.2

# 添加go 语言插件

# 下载插件

ADD https://marketplace.visualstudio.com/_apis/public/gallery/publishers/golang/vsextensions/Go/0.26.0/vspackage /root/
RUN ${code_server}/bin/code-server --install-extension /root/golang.Go-0.26.0.vsix
#formulahendry.code-runner.0.11.5.vsix
ADD https://marketplace.visualstudio.com/_apis/public/gallery/publishers/formulahendry/vsextensions/code-runner/0.11.5/vspackage /root/
RUN ${code_server}/bin/code-server --install-extension /root/formulahendry.code-runner.0.11.5.vsix




RUN /usr/local/go/bin/go get -v github.com/uudashr/gopkgs/v2/cmd/gopkgs
RUN /usr/local/go/bin/go get -v github.com/ramya-rao-a/go-outline
RUN /usr/local/go/bin/go get -v github.com/cweill/gotests/gotests
RUN /usr/local/go/bin/go get -v github.com/fatih/gomodifytags
RUN /usr/local/go/bin/go get -v github.com/josharian/impl
RUN /usr/local/go/bin/go get -v github.com/haya14busa/goplay/cmd/goplay
RUN /usr/local/go/bin/go get -v github.com/go-delve/delve/cmd/dlv
RUN /usr/local/go/bin/go get -v -d github.com/go-delve/delve/cmd/dlv@master
RUN /usr/local/go/bin/go get -v honnef.co/go/tools/cmd/staticcheck
RUN /usr/local/go/bin/go get -v golang.org/x/tools/gopls

# 安装git
RUN apt-get update
RUN apt-get -y install git

# 启动服务器

EXPOSE 8080

ENTRYPOINT ${code_server}/bin/code-server --host 0.0.0.0 --port 8080

