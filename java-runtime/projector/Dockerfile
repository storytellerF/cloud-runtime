FROM ubuntu:20.04

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install python3 python3-pip -y
RUN apt-get install python3-cryptography -y

RUN python3 -m pip install -U pip 

RUN apt-get install less libxext6 libxrender1 libxtst6 libfreetype6 libxi6 -y  

RUN pip3 install projector-installer --user

RUN source ~/.profile 

CMD projector find
