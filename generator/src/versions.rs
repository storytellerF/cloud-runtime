use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
   pub versions: Versions
}

#[derive(Debug, Deserialize)]
pub struct Versions {
   pub code_runner: String
}