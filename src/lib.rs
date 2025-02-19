use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg};

// 配置结构体，存储命令行参数

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,   // 要处理的文件列表
    lines: usize,         // 要显示的行数
    bytes: Option<usize>, // 要显示的字节数（可选）
}

/// 解析命令行参数并返回配置
pub fn get_args() -> Result<Config> {
    // 创建命令行应用程序
    let matches = App::new("header")
        .version("0.1.0")
        //.author("Your Name <you@example.com>")
        .about("Rust version of the 'head' command")
        // 设置 -n/--lines 参数，用于指定显示的行数
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines to show")
                .default_value("10"), // 默认显示10行
        )
        // 设置 -c/--bytes 参数，用于指定显示的字节数
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines") // bytes 参数和 lines 参数互斥
                .help("Number of bytes to show"),
        )
        // 设置文件参数，可以接收多个文件
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true) // 允许多个文件
                .default_value("-"), // 默认从标准输入读取
        )
        .get_matches();

    //定义闭包来解析正整数
    let parse_positive_int = |s: &str| -> Result<usize> {
        match s.parse() {
            Ok(n) if n > 0 => Ok(n),
            _ => Err(anyhow!("illegal number: {}", s)),
        }
    };
    /*
     * transpose 函数的作用：
     * 当遇到 `Some(Ok(v))` 时，会返回 `Ok(Some(v))`
     * 当遇到 `Some(Err(e))` 时，会返回 `Err(e)`
     * 当遇到 `None` 时，会返回 `Ok(None)`
     *
     */
    // 解析 lines 参数
    let lines = matches
        .value_of("lines") // 获取 lines 参数的值
        .map(parse_positive_int) // 将值转换为正整数
        .transpose() // 将结果转换为 Option<usize>
        .context(format!(
            "Failed to parse lines count: {}",
            matches.value_of("lines").unwrap_or("unknown")
        ))?; // 如果转换失败，返回错误并包含失败的字符串

    // 解析 bytes 参数
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .context(format!(
            "Failed to parse bytes count: {}",
            matches.value_of("bytes").unwrap_or("unknown")
        ))?;

    // 获取文件列表
    let files = matches.values_of_lossy("files").unwrap_or_default();

    // 返回配置对象
    Ok(Config {
        files,
        lines: lines.unwrap_or(10),
        bytes,
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).context(format!("Failed to open file: {}", filename))?,
        ))),
    }
}

/// 运行程序的主要逻辑
pub fn run(config: Config) -> Result<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                //多个文件处理
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}
