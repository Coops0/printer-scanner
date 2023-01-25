use anyhow::Result;
use clap::Parser;
use crate::scanner::scan_for_devices;

mod scanner;
mod threads;
mod util;
mod id;

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
    #[arg(short, long, default_value_t = 2000)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    scan_for_devices(args).await
}
