
use std::io::Write;

#[derive(Debug, Copy, Clone)]
pub struct Plugin<'a> {
    pub plugin_key: &'a str,
    pub plugin_name: &'a str,
    pub author_name: &'a str,
    pub plugin_version: &'a str,
}
pub fn setup_coder(mut file: std::fs::File, plugins: Vec<Plugin>) {
    let v = plugins.iter().map(|&x| include_plugin(x)).collect::<Vec<_>>();
    let plugin_string = v.join("\n");
    file.write_all("\nARG code_server_parent=/usr/local
ARG code_server_version=4.14.1
ARG code_server_flavor=linux-arm64
ARG code_server_bin=code-server-${code_server_version}-${code_server_flavor}
ARG code_server_executable=${code_server_parent}/${code_server_bin}/bin/code-server

#code server
WORKDIR ${code_server_parent}
RUN wget https://github.com/coder/code-server/releases/download/v${code_server_version}/${code_server_bin}.tar.gz
RUN tar -xzf ${code_server_bin}.tar.gz

{plugins}

COPY start.sh /
RUN chmod +x /start.sh && echo /usr/local/code-server-4.14.1-linux-arm64/bin/code-server > /start.sh
ENTRYPOINT /start.sh".replace("{plugins}", &plugin_string).as_bytes()).expect("write failed");
}

pub fn include_plugin(plugin: Plugin)  -> String {
    return install_plugin(plugin.plugin_key, plugin.author_name, plugin.plugin_name, plugin.plugin_version)
}

pub fn install_plugin(plugin_key: &str, author_name: &str, plugin_name: &str, plugin_version: &str) -> String {
    let str = String::from("#安装{plugin_key}
WORKDIR /root/
ARG {plugin_key}_version={plugin_version}
ARG {plugin_key}_author={author_name}
ARG {plugin_key}_artifact={plugin_name}
ARG {plugin_key}_name=${plugin_key}_author.${plugin_key}_artifact-${{plugin_key}_version}.vsix
RUN wget https://open-vsx.org/api/${plugin_key}_author/${plugin_key}_artifact/${{plugin_key}_version}/file/${plugin_key}_name
RUN ${code_server_executable} --install-extension /root/${plugin_key}_name
#安装{plugin_key}结束\n").replace("{plugin_key}", plugin_key).replace("{author_name}", author_name).replace("{plugin_name}", plugin_name).replace("{plugin_version}", plugin_version);
    // let bytes = str.into_bytes();
    // return Box::leak(bytes.into_boxed_slice());
    return str;
}