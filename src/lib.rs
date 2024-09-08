mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, ChaCha20SubCommand, Cli, HttpSubcommand, SignFormat,
    SubCommand, TextSubCommand,
};
pub use process::*;
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
