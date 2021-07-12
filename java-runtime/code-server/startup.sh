docker run -it --name code-server-java -p 8080:8080 \
  -v "$HOME/.config:/root/.config" \
  -v "/root/project:/root/project" \
  -u "$(id -u):$(id -g)" \
  -e "DOCKER_USER=$USER" \
  test:latest
# 需要修改config.yaml 的绑定地址
#bind-addr: 127.0.0.1:8080
#auth: password
#password: 30c40fe5b0d1f735a3e7309e
#cert: false
