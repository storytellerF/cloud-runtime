# 开箱即用的ide

基于code server <https://github.com/coder/code-server>

## 端口分配

| Language     | PORT   |
|--------------|------|
| Android      | 8198 |
| C            | 8199 |
| Go           | 8200 |
| Java-Gradle  | 8201 |
| Rust         | 8202 |
| Tang         | 8203 |
| Python       | 8204 |

## Bind

默认情况下绑定的都是数据卷，如果需要绑定本地路径需要手动修改**docker-compose.yml**

```yml
services:
  rust-dev:
    build: 
      context: ../context
      dockerfile: ../rust-runtime/code-server-based/Dockerfile
    volumes:
      - $HOME/.config:/root/.config
      - $HOME/Projects:/root/project
    ports:
      - 8202:8080
    command: --bind-addr 0.0.0.0:8080
```

## Build

[generator](generator) 项目是rust 项目，可以通过**rust-runtime** 构建出环境后编译。

```shell
cd generator
cargo run
```