use std::{ffi::OsStr, path::Path};

use clap::Parser;
use url::Url;

mod web2image;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "CarlJin", version = "0.1.0", about = "take screenshot of url and generete a image with qrcode", long_about = None)]
struct Args {
    /// 需要生成的链接
    #[clap(index = 1, required = true, validator = valid_url)]
    url: String,

    /// 图片存放路径
    #[clap(index = 2, default_value = "/tmp/snapshot.jpg", validator = valid_output_path)]
    output: String,
}

fn get_file_ext(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|p| {
            let ext = p.to_lowercase();
            match ext.as_str() {
                "jpg" | "png" => Some(ext),
                _ => None,
            }
        })
}

//  验证是否输入正确的 output 是否合法
fn valid_output_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    let ext = get_file_ext(path);
    let has_root = path.has_root();
    let is_relative = path.is_relative();
    let has_parent = parent.is_some();
    let exit_passed = ext.is_some();

    //   ./abc.jpg  abc.jpg
    if is_relative && exit_passed && !has_root {
        return Ok(());
    }

    if has_parent && exit_passed {
        return Ok(());
    }

    Err(String::from("输入错误的 output 值"))
}

//  判断输入的 url 是否合法
fn valid_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("错误的 url")),
    }
}

fn main() {
    let args = Args::parse();

    println!("Hello {:#?}!", args);

    web2image::web2image(&args.url, &args.output).expect("执行错误");
}
