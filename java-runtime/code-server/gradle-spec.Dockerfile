FROM ubuntu:latest
#区别是不使用maven

#ENV http_proxy http://192.168.174.1:10811
#ENV https_proxy https://192.168.174.1:10811
#ENV no_proxy="127.0.0.1,localhost"
#code server 版本4.4.0

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install wget unzip -y
# 下载jdk
WORKDIR /usr/local
RUN wget https://github.com/adoptium/temurin16-binaries/releases/download/jdk-16.0.2%2B7/OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz
# 解压出来的目录是jdk-16.0.2+7
RUN tar -xzf OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz

#配置jdk 环境变量
ENV PATH="/usr/local/jdk-16.0.2+7/bin:${PATH}"

#gradle。如果有疑问可以参考https://gradle.org/install/
RUN mkdir /opt/gradle
WORKDIR /opt/gradle
RUN wget https://services.gradle.org/distributions/gradle-7.4.2-bin.zip
RUN unzip -q gradle-7.4.2-bin.zip

ENV PATH="/opt/gradle/gradle-7.4.2-bin/bin:${PATH}"

#code server
WORKDIR /usr/local/
RUN wget https://github.com/coder/code-server/releases/download/v4.4.0/code-server-4.4.0-linux-amd64.tar.gz
RUN tar -xzf code-server-4.4.0-linux-amd64.tar.gz

# 安装java插件包
WORKDIR /root/
RUN wget vscjava.vscode-java-pack-0.22.4.vsix
RUN /usr/local/code-server-4.4.0-linux-amd64/bin/code-server --install-extension /root/vscjava.vscode-java-pack-0.22.4.vsix

# 安装gradle插件
WORKDIR /root/
RUN wget https://open-vsx.org/api/richardwillis/vscode-gradle/3.6.1/file/richardwillis.vscode-gradle-3.6.1.vsix
RUN /usr/local/code-server-4.4.0-linux-amd64/bin/code-server --install-extension /root/richardwillis.vscode-gradle-3.6.1.vsix

RUN wget https://open-vsx.org/api/redhat/fabric8-analytics/0.3.5/file/redhat.fabric8-analytics-0.3.5.vsix
RUN /usr/local/code-server-4.4.0-linux-amd64/bin/code-server --install-extension /root/redhat.fabric8-analytics-0.3.5.vsix

RUN wget https://open-vsx.org/api/redhat/vscode-xml/0.20.0/file/redhat.vscode-xml-0.20.0.vsix
RUN /usr/local/code-server-4.4.0-linux-amd64/bin/code-server --install-extension /root/redhat.vscode-xml-0.20.0.vsix


#暴露端口
EXPOSE 8080

ENTRYPOINT /usr/local/code-server-4.4.0-linux-amd64/bin/code-server
