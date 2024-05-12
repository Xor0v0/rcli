use clap::Parser;

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
