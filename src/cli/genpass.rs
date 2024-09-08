use crate::{process_genpass, CmdExecutor};
use anyhow::Ok;
use clap::Parser;
use zxcvbn::zxcvbn;

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

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let passwd = process_genpass(
            self.length,
            self.noupper,
            self.nolower,
            self.nonumber,
            self.nosymbol,
        )?;
        println!("{}", passwd);
        eprintln!("{}", zxcvbn(&passwd, &[]).score());
        Ok(())
    }
}
