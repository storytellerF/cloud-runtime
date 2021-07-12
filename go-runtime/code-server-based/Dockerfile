FROM ubuntu:20.04 
#与default 中的不同，default中的继续go-lang的镜像。

#ADD https://github.com/cdr/code-server/releases/download/v3.10.2/code-server-3.10.2-linux-amd64.tar.gz /usr/local/code-server/
#WORKDIR /usr/local/code-server/
#RUN tar -xzf code-server-3.10.2-linux-amd64.tar.gz

ADD code-server-3.10.2-linux-amd64.tar.gz /usr/local/code-server/

ADD golang.Go-0.26.0.vsix /root/

ENV code_server /usr/local/code-server/code-server-3.10.2-linux-amd64

RUN ${code_server}/bin/code-server --install-extension /root/golang.Go-0.26.0.vsix

EXPOSE 8080

ENTRYPOINT ${code_server}/bin/code-server