#!/bin/bash

# 获取当前系统的Ubuntu版本号
VERSION=$(lsb_release -sr)

# 从官方镜像站获取可用的镜像列表，保存到临时文件中
curl -s "https://launchpad.net/ubuntu/+cdmirrors?memo=20&start=0" | grep -P -o "(?<=href=\").*?(?=\")|(?<=href=\').*?(?=\')" | grep "http" | grep "releases.ubuntu.com" > /tmp/mirrors.txt

# 计算每个镜像站的响应时间，并将结果保存到临时文件中
echo "Testing mirror speeds..."
> /tmp/speed.txt
while read line; do
    speed=$(curl -s -w %{speed_download} -o /dev/null ${line}/dists/focal/Release)
    echo "$speed $line" >> /tmp/speed.txt
done < /tmp/mirrors.txt

# 按照响应时间排序，选择最快的镜像站并更新APT源
fastest=$(sort -n /tmp/speed.txt | head -n 1 | awk '{print $2}')
echo "Fastest mirror: $fastest"
sudo sed -i "s|http://.*.ubuntu.com/ubuntu/|${fastest}/ubuntu/|g" /etc/apt/sources.list

# 清理临时文件
rm /tmp/mirrors.txt /tmp/speed.txt