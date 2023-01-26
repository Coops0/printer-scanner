use std::env;

use anyhow::Result;
use clap::Parser;

use crate::printing::{ipp_info, print, print_ipp};
use crate::scanner::scan_for_devices;

mod id;
mod scanner;
mod threads;
mod util;
mod printing;

#[derive(Parser, Debug, Clone)]
#[command(long_about = None)]
pub struct Args {
    /// Amount of threads to simultaneously request on
    #[arg(short, long, default_value_t = 20)]
    threads: usize,

    /// Log failures as well
    #[arg(short, long)]
    verbose: bool,

    /// The subnet to generate ips for (use x to denote a wildcard)
    #[arg(short, long, default_value_t = String::from("10.208.x.x"))]
    ip_subnet: String,

    /// Display a progress bar
    #[arg(short, long, default_value_t = true)]
    progress_bar: bool,

    /// Append ips to file after each success instead of all at the end
    #[arg(short, long)]
    append_file: bool,

    /// Timeout for scanning (in ms)
    #[arg(long, default_value_t = 2000)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::clone).unwrap_or_default().as_str() {
        "printcmd" => {
            let e = print(&args[2], &args[3]).await?;
            println!("{e}");
            return Ok(());
        }
        "getinfo" => {
            return ipp_info(&args[2]).await;
        }
        "print" => {
            return print_ipp(&args[2], &args[3]).await;
        }
        _ => {}
    }

    let args = Args::parse();
    scan_for_devices(args).await
}
