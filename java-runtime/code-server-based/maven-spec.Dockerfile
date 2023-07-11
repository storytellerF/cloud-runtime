FROM ubuntu:latest
LABEL maintainer="storytellerF"
LABEL version="1.0"
LABEL description="java maven"
#ENV http_proxy http://192.168.174.1:10811
#ENV https_proxy https://192.168.174.1:10811
#ENV no_proxy="127.0.0.1,localhost"
#code server 版本4.4.0

ENV code_server_version=4.4.0
ENV code_server_flavor=${code_server_version}-linux-arm64
ENV code_server_executable=/usr/local/code-server-${code_server_flavor}/bin/code-server

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install wget -y
# 下载jdk
WORKDIR /usr/local
RUN wget https://github.com/adoptium/temurin16-binaries/releases/download/jdk-16.0.2%2B7/OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz
# 解压出来的目录是jdk-16.0.2+7
RUN tar -xzf OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz

#配置jdk 环境变量
ENV PATH="/usr/local/jdk-16.0.2+7/bin:${PATH}"

ARG maven_version=3.8.5
RUN mkdir /opt/maven
WORKDIR /opt/maven
RUN wget https://dlcdn.apache.org/maven/maven-3/${maven_version}/binaries/apache-maven-${maven_version}-bin.tar.gz
RUN tar -xzf apache-maven-${maven_version}-bin.tar.gz
ENV PATH="/opt/maven/apache-maven-${maven_version}-bin/bin:${PATH}"

#code server
WORKDIR /usr/local/
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/code-server-${code_server_flavor}.tar.gz
RUN tar -xzf code-server-${code_server_flavor}.tar.gz

# 安装java插件包
WORKDIR /root/
ARG java_pack_version=0.22.4
RUN wget https://open-vsx.org/api/vscjava/vscode-java-pack/${java_pack_version}/file/vscjava.vscode-java-pack-${java_pack_version}.vsix
RUN ${code_server_executable} --install-extension /root/vscjava.vscode-java-pack-${java_pack_version}.vsix

ARG java_analytics_version=0.3.5
RUN wget https://open-vsx.org/api/redhat/fabric8-analytics/${java_analytics_version}/file/redhat.fabric8-analytics-${java_analytics_version}.vsix
RUN ${code_server_executable} --install-extension /root/redhat.fabric8-analytics-${java_analytics_version}.vsix

ARG vscode_xml_version=0.20.0
RUN wget https://open-vsx.org/api/redhat/vscode-xml/${vscode_xml_version}/file/redhat.vscode-xml-${vscode_xml_version}.vsix
RUN ${code_server_executable} --install-extension /root/redhat.vscode-xml-${vscode_xml_version}.vsix

#暴露端口
EXPOSE 8080

ENTRYPOINT ${code_server_executable}
