
use std::io::Write;

#[path = "coder.rs"]
mod coder;
#[path ="common.rs"]
mod common;

pub fn write() {
    let mut file = common::file_instance("../android-runtime/code-server-based/Dockerfile");
    file.write_all(common::ubuntu(vec!["openjdk-17-jdk", "unzip"])).expect("write failed");
    file.write_all(
        "
ARG sdkName=android-sdk
ARG homeDir=/root
ARG unzipName=cmdline-tools
ARG latestDir=latest
ARG name
ENV ANDROID_HOME=$homeDir/${sdkName}/

WORKDIR $homeDir
ARG zipName=commandlinetools-linux-9477386_latest.zip
RUN wget https://dl.google.com/android/repository/${zipName}
RUN unzip -q $zipName -d $sdkName

WORKDIR ${homeDir}/${sdkName}/${unzipName}
RUN mkdir ${latestDir} && mv bin lib NOTICE.txt source.properties ${latestDir}
WORKDIR ${homeDir}/${sdkName}/${unzipName}/$latestDir/bin
RUN ls
RUN yes | ./sdkmanager --licenses
#仅安装platfo-tools，不安装build-tools，这个在编译项目的时候会自动下载，具体下载的版本由Android 自动选择。
RUN ./sdkmanager platform-tools

# Install libs so Android's AAPT2 will run on an arm64 arch
RUN apt-get update && apt-get install -y libc6-amd64-cross libgcc1-amd64-cross && ln -s /usr/x86_64-linux-gnu/lib64/ /lib64
ENV LD_LIBRARY_PATH=\"$LD_LIBRARY_PATH:/lib64:/usr/x86_64-linux-gnu/lib\"\n"
            .as_bytes(),
    )
    .expect("write failed");
    file.write_all(coder::setup_coder(vec![]).as_bytes()).expect("write failed");
}