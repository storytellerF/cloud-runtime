use crate::versions;

#[derive(Debug, Copy, Clone)]
pub struct Plugin<'a> {
    pub plugin_key: &'a str, //需要使用下划线作为分隔符
    pub plugin_name: &'a str,
    pub author_name: &'a str,
    pub plugin_version: &'a str,
}
pub fn setup_coder<'a,'b>(mut plugins: Vec<Plugin<'b>>, config: &'b versions::Config) -> String {
    plugins.insert(
        0,
        Plugin {
            plugin_key: "git_ignore_plugin",
            author_name: "codezombiech",
            plugin_name: "gitignore",
            plugin_version: &config.versions.git_ignore,
        },
    );
    plugins.insert(
        0,
        Plugin {
            plugin_key: "git_lens",
            plugin_name: "gitlens",
            author_name: "eamodio",
            plugin_version: &config.versions.git_lens,
        },
    );
    let v = plugins
        .iter()
        .map(|&x| include_plugin(x))
        .collect::<Vec<_>>();
    let plugin_string = v.join("\n");
    return "
ARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_pack_name=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_sub_path=bin/code-server
#ARG code_server_executable=${code_server_parent}/${code_server_pack_name}/${code_server_sub_path}
ARG code_server_executable=/usr/${code_server_sub_path}

#code server
#WORKDIR ${code_server_parent}
#RUN curl -LO https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_pack_name}.tar.gz
#RUN tar -xzf ${code_server_pack_name}.tar.gz

RUN curl -fsSL https://code-server.dev/install.sh | sh 

{plugins}

RUN echo $code_server_executable --bind-addr 0.0.0.0:8080 > /start.sh && chmod +x /start.sh
ENTRYPOINT /start.sh".replace("{plugins}", &plugin_string);
}

pub fn include_plugin(plugin: Plugin) -> String {
    return install_plugin(
        plugin.plugin_key,
        plugin.author_name,
        plugin.plugin_name,
        plugin.plugin_version,
    );
}

pub fn install_plugin(
    plugin_key: &str,
    author_name: &str,
    plugin_name: &str,
    plugin_version: &str,
) -> String {
    let str = String::from("#安装{plugin_key}
WORKDIR /root/
ARG {plugin_key}_version={plugin_version}
ARG {plugin_key}_author={author_name}
ARG {plugin_key}_artifact={plugin_name}
ARG {plugin_key}_name=${plugin_key}_author.${plugin_key}_artifact-${{plugin_key}_version}.vsix
RUN curl -LO https://open-vsx.org/api/${plugin_key}_author/${plugin_key}_artifact/${{plugin_key}_version}/file/${plugin_key}_name
RUN ${code_server_executable} --install-extension /root/${plugin_key}_name
#安装{plugin_key}结束\n").replace("{plugin_key}", plugin_key).replace("{author_name}", author_name).replace("{plugin_name}", plugin_name).replace("{plugin_version}", plugin_version);
    // let bytes = str.into_bytes();
    // return Box::leak(bytes.into_boxed_slice());
    return str;
}
