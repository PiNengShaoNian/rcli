use crate::utils::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
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
