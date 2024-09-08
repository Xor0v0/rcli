use super::verify_path;
use crate::{process_http_serve, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[enum_dispatch(CmdExecutor)]
#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(name = "serve", about = "Start a HTTP server")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}
