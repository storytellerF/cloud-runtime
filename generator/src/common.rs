use std::{path::Path, fs::{self, File}};

pub fn ubuntu(extra: Vec<&str>) -> &[u8] {
    let extras = extra.join(" ");
    let base = String::from(
        "FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl git vim "
    ) + &extras;
    let bytes = base.into_bytes();
    return Box::leak(bytes.into_boxed_slice());
}

pub fn fileInstance(path: &str) -> File {
    let path = Path::new(path);
    let parent = path.parent().unwrap();
    
    // 如果父目录不存在，则创建该目录
    if !parent.exists() {
        fs::create_dir_all(parent).expect("创建失败");
    }
    let file = File::create(path).expect("创建文件失败");
    return file;
}