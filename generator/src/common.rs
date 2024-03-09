use std::{
    fs::{self, File},
    path::Path,
};

pub fn ubuntu(extra: Vec<&str>) -> &[u8] {
    let extras = extra.join(" ");
    let base = String::from(
        "FROM ubuntu:latest
RUN apt-get update && apt-get -y install ca-certificates
ARG sourcesRaw=sources.list.prefer
COPY ./${sourcesRaw}.a /etc/apt/${sourcesRaw}.a
COPY ./${sourcesRaw}.x /etc/apt/${sourcesRaw}.x

WORKDIR /etc/apt
RUN mv sources.list sources.list.backup \\
        && cp ${sourcesRaw}.$(uname -m | cut -c1) sources.list \\
        && apt-get update && apt-get upgrade -y && apt-get install -y curl git vim \\
        && apt-get install -y ",
    ) + &extras;
    let bytes = base.into_bytes();
    return Box::leak(bytes.into_boxed_slice());
}

pub fn file_instance(path: &str) -> File {
    let path = Path::new(path);
    let parent = path.parent().unwrap();

    // 如果父目录不存在，则创建该目录
    if !parent.exists() {
        fs::create_dir_all(parent).expect("创建失败");
    }
    let file = File::create(path).expect("创建文件失败");
    return file;
}

pub fn manual_install_jdk() -> &'static [u8] {
    return "
ARG jdkInstallPath=/usr/local
ARG jdkDownloadName=OpenJDK16U-jdk_x64_linux_hotspot_16.0.2_7.tar.gz
ARG jdkName=jdk-16.0.2+7
ARG jdkNameEncoded=jdk-16.0.2%2B7
# 下载jdk
# 解压出来的目录是jdk-16.0.2+7
WORKDIR $jdkInstallPath
RUN curl -LO https://github.com/adoptium/temurin16-binaries/releases/download/$jdkNameEncoded/$jdkDownloadName \\
    && tar -xzf $jdkDownloadName
#配置jdk 环境变量
ENV JAVA_HOME=$jdkInstallPath/$jdkName".as_bytes();
}

pub fn manual_install_gradle() -> &'static [u8] {
    return "#gradle。如果有疑问可以参考https://gradle.org/install/
ARG gradle_version=7.4.2
ARG gradleInstallPath=/opt/gradle
RUN mkdir $gradleInstallPath
WORKDIR $gradleInstallPath
RUN curl -LO https://services.gradle.org/distributions/gradle-${gradle_version}-bin.zip \\
    && unzip -q gradle-${gradle_version}-bin.zip"
        .as_bytes();
}

pub fn manual_install_maven() -> &'static [u8] {
    return "ARG maven_version=3.8.5
ARG mavenInstallPath=/opt/maven
RUN mkdir $mavenInstallPath
WORKDIR $mavenInstallPath
RUN curl -LO https://dlcdn.apache.org/maven/maven-3/${maven_version}/binaries/apache-maven-${maven_version}-bin.tar.gz ||
    && tar -xzf apache-maven-${maven_version}-bin.tar.gz".as_bytes();
}
