
mod client;
mod server;
mod error;

use error::Error;
use client::cli::{main as cli, main_save, main_ts, main_yt};
use server::{run_api_server, run_webui_server};

type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()> {
    // TODO: depending on command line options, start client or server
    println!("Hello, world!");
    
    cli()?;
    Ok(())
}
