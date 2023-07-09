FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y wget curl unzip git openjdk-17-jdk vim 

ARG sdkName=android-sdk
ARG homeDir="/root"
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
RUN ./sdkmanager "platform-tools"

# Install libs so Android's AAPT2 will run on an arm64 arch
RUN apt-get update && apt-get install -y libc6-amd64-cross libgcc1-amd64-cross && ln -s /usr/x86_64-linux-gnu/lib64/ /lib64
ENV LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/lib64:/usr/x86_64-linux-gnu/lib"

ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz
COPY start.sh /
RUN chmod +x /start.sh && echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh
ENTRYPOINT /start.sh