FROM ubuntu:latest
RUN apt-get update && apt-get -y install ca-certificates
ARG sourcesRaw=sources.list.prefer
COPY ./${sourcesRaw}.a /etc/apt/${sourcesRaw}.a
COPY ./${sourcesRaw}.x /etc/apt/${sourcesRaw}.x

WORKDIR /etc/apt
RUN mv sources.list sources.list.backup \
        && cp ${sourcesRaw}.$(uname -m | cut -c1) sources.list \
        && apt-get update && apt-get upgrade -y && apt-get install -y curl git vim \
        && apt-get install -y 