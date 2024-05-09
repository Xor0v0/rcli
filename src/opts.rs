use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about = None, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, default_value = "json", value_parser = parse_format)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

// 密码必定有字母, 用户指定密码是否支持: 长度, 数字, 大小写, 符号
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub nonumber: bool,

    #[arg(long, default_value_t = false)]
    pub nolower: bool,

    #[arg(long, default_value_t = false)]
    pub noupper: bool,

    #[arg(long, default_value_t = false)]
    pub nosymbol: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists".into())
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // parse() 可以把一个 &str 解析成其他类型, 但是需要实现 FromStr trait
    format.parse()
}

// 由于采用了 `default_value` 方法, 所以需要实现 From trait
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 由于采用了 parse() 方法, 可以将 &str 类型解析成其他类型
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => Err(anyhow::anyhow!("Unsupported format: {}", v)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
