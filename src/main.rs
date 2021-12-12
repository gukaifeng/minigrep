/*
author: gukaifeng
email: 892859816@qq.com
github: https://github.com/gukaifeng/minigrep/
blog: https://gukaifeng.cn/
*/

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applecation error: {}", e);
        process::exit(1);
    }
}
