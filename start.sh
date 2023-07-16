docker run -it --name go-runtime -p 8080:8080 \
  -v "$HOME/.config:/root/.config" \
  -v "/root/project:/root/project" \
  -u "$(id -u):$(id -g)" \
  -e "DOCKER_USER=$USER" \
  codercom/code-server:latest