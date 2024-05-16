FROM ubuntu:latest
RUN apt-get update && apt-get -y install ca-certificates
ARG sourcesRaw=sources.list.prefer
COPY ./${sourcesRaw}.a /etc/apt/${sourcesRaw}.a
COPY ./${sourcesRaw}.x /etc/apt/${sourcesRaw}.x

WORKDIR /etc/apt
RUN mv sources.list sources.list.backup \
        && cp ${sourcesRaw}.$(uname -m | cut -c1) sources.list \
        && apt-get update && apt-get upgrade -y && apt-get install -y curl git vim \
        && apt-get install -y unzip
ARG jdkInstallPath=/usr/local
ARG jdkDownloadName=OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz
ARG jdkName=jdk-16.0.2+7
ARG jdkNameEncoded=jdk-16.0.2%2B7
# 下载jdk
# 解压出来的目录是jdk-16.0.2+7
WORKDIR $jdkInstallPath
RUN curl -LO https://github.com/adoptium/temurin16-binaries/releases/download/$jdkNameEncoded/$jdkDownloadName \
    && tar -xzf $jdkDownloadName
#配置jdk 环境变量
ENV JAVA_HOME=$jdkInstallPath/$jdkNameARG maven_version=3.8.5
ARG mavenInstallPath=/opt/maven
RUN mkdir $mavenInstallPath
WORKDIR $mavenInstallPath
RUN curl -LO https://dlcdn.apache.org/maven/maven-3/${maven_version}/binaries/apache-maven-${maven_version}-bin.tar.gz ||
    && tar -xzf apache-maven-${maven_version}-bin.tar.gz