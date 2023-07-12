FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git vim unzip
ARG jdkInstallPath=/usr/local
ARG jdkDownloadName=OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz
ARG jdkName=jdk-16.0.2+7
ARG jdkNameEncoded=jdk-16.0.2%2B7
# 下载jdk
WORKDIR $jdkInstallPath
RUN curl -LO https://github.com/adoptium/temurin16-binaries/releases/download/$jdkNameEncoded/$jdkDownloadName
# 解压出来的目录是jdk-16.0.2+7
RUN tar -xzf $jdkDownloadName
#配置jdk 环境变量
ENV JAVA_HOME=$jdkInstallPath/$jdkNameARG maven_version=3.8.5
ARG mavenInstallPath=/opt/maven
RUN mkdir $mavenInstallPath
WORKDIR $mavenInstallPath
RUN curl -LO https://dlcdn.apache.org/maven/maven-3/${maven_version}/binaries/apache-maven-${maven_version}-bin.tar.gz
RUN tar -xzf apache-maven-${maven_version}-bin.tar.gz
ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN curl -LO https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz

#安装git_lens
WORKDIR /root/
ARG git_lens_version=14.0.1
ARG git_lens_author=eamodio
ARG git_lens_artifact=gitlens
ARG git_lens_name=$git_lens_author.$git_lens_artifact-${git_lens_version}.vsix
RUN curl -LO https://open-vsx.org/api/$git_lens_author/$git_lens_artifact/${git_lens_version}/file/$git_lens_name
RUN ${code_server_executable} --install-extension /root/$git_lens_name
#安装git_lens结束

#安装git_ignore_plugin
WORKDIR /root/
ARG git_ignore_plugin_version=0.9.0
ARG git_ignore_plugin_author=codezombiech
ARG git_ignore_plugin_artifact=gitignore
ARG git_ignore_plugin_name=$git_ignore_plugin_author.$git_ignore_plugin_artifact-${git_ignore_plugin_version}.vsix
RUN curl -LO https://open-vsx.org/api/$git_ignore_plugin_author/$git_ignore_plugin_artifact/${git_ignore_plugin_version}/file/$git_ignore_plugin_name
RUN ${code_server_executable} --install-extension /root/$git_ignore_plugin_name
#安装git_ignore_plugin结束

#安装vscjava
WORKDIR /root/
ARG vscjava_version=0.22.4
ARG vscjava_author=vscjava
ARG vscjava_artifact=vscode-java-pack
ARG vscjava_name=$vscjava_author.$vscjava_artifact-${vscjava_version}.vsix
RUN curl -LO https://open-vsx.org/api/$vscjava_author/$vscjava_artifact/${vscjava_version}/file/$vscjava_name
RUN ${code_server_executable} --install-extension /root/$vscjava_name
#安装vscjava结束

#安装redhat_analytics
WORKDIR /root/
ARG redhat_analytics_version=0.3.5
ARG redhat_analytics_author=redhat
ARG redhat_analytics_artifact=fabric8-analytics
ARG redhat_analytics_name=$redhat_analytics_author.$redhat_analytics_artifact-${redhat_analytics_version}.vsix
RUN curl -LO https://open-vsx.org/api/$redhat_analytics_author/$redhat_analytics_artifact/${redhat_analytics_version}/file/$redhat_analytics_name
RUN ${code_server_executable} --install-extension /root/$redhat_analytics_name
#安装redhat_analytics结束

#安装redhat_xml
WORKDIR /root/
ARG redhat_xml_version=0.20.0
ARG redhat_xml_author=redhat
ARG redhat_xml_artifact=vscode-xml
ARG redhat_xml_name=$redhat_xml_author.$redhat_xml_artifact-${redhat_xml_version}.vsix
RUN curl -LO https://open-vsx.org/api/$redhat_xml_author/$redhat_xml_artifact/${redhat_xml_version}/file/$redhat_xml_name
RUN ${code_server_executable} --install-extension /root/$redhat_xml_name
#安装redhat_xml结束


RUN echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh && chmod +x /start.sh
ENTRYPOINT /start.sh