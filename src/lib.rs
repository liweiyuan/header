use clap::{App, Arg};

// 自定义错误类型，用于处理程序中可能出现的所有错误
type HeaderResult<T> = Result<T, Box<dyn std::error::Error>>;

// 配置结构体，存储命令行参数
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,   // 要处理的文件列表
    lines: usize,         // 要显示的行数
    bytes: Option<usize>, // 要显示的字节数（可选）
}

/// 解析命令行参数并返回配置
pub fn get_args() -> HeaderResult<Config> {
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
        .map_err(|e| format!("illegal line count ... {}", e))?; // 如果转换失败，返回错误

    // 解析 bytes 参数
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count ... {}", e))?;

    // 获取文件列表
    let files = matches.values_of_lossy("files").unwrap_or_default();

    // 返回配置对象
    Ok(Config {
        files,
        lines: lines.unwrap(),
        bytes: bytes,
    })
}

/// 解析正整数，确保输入的数字大于0 普通函数自动实现Fn FnOnce FnMut
fn parse_positive_int(s: &str) -> HeaderResult<usize> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(s)), // 解析失败或数字小于等于0时返回错误
    }
}

/// 运行程序的主要逻辑
pub fn run(config: Config) -> HeaderResult<()> {
    // TODO: 实现文件读取和内容显示的逻辑
    Ok(())
}
