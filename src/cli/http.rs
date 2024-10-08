use crate::{process_http_serve, utils::verify_path, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(long, value_parser = verify_path, default_value = ".")]
    pub path: PathBuf,
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.path, self.port).await
    }
}
