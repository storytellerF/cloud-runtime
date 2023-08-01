use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
   pub versions: Versions
}

#[derive(Debug, Deserialize)]
pub struct Versions {
   pub code_runner: String,
   pub git_lens: String,
   pub git_ignore: String,
   pub go_lang: String,
   pub redhat_xml: String,
   pub redhat_analytics: String,
   pub java_pack: String,
   pub vsc_python: String,
   pub rust_pack: String
}