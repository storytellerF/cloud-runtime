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
ENV JAVA_HOME=$jdkInstallPath/$jdkName#gradle。如果有疑问可以参考https://gradle.org/install/
ARG gradle_version=7.4.2
ARG gradleInstallPath=/opt/gradle
RUN mkdir $gradleInstallPath
WORKDIR $gradleInstallPath
RUN curl -LO https://services.gradle.org/distributions/gradle-${gradle_version}-bin.zip \
    && unzip -q gradle-${gradle_version}-bin.zip