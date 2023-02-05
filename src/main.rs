use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::printing::print_ipp;
use crate::scanner::scan_for_devices;

mod id;
mod printing;
mod scanner;
mod threads;
mod util;
mod printer;

#[derive(clap::Args, Debug, Clone)]
#[command(long_about = None)]
pub struct ScannerArgs {
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

#[derive(clap::Args, Debug, Clone)]
#[command(long_about = None)]
pub struct PrintArgs {
    /// ip to print to, e.x. http://10.208.2.22
    #[arg(short, long)]
    ip: String,

    /// file path to print, can be local or absolute
    #[arg(short, long)]
    file: String,

    /// number of copies to print
    #[arg(short, long, default_value_t = 1)]
    copies: u32,

    /// bypass file extension check
    #[arg(short, long)]
    bypass_ext: bool,

    /// automatically identify printer accepted formats
    #[arg(long, default_value_t = true)]
    identify_formats: bool,

    #[arg(short, long)]
    only_detect_formats: bool,
}

#[derive(Parser, Debug, Clone)]
#[command(long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    Scan(ScannerArgs),
    Print(PrintArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Scan(s) => scan_for_devices(s).await,
        Action::Print(p) => print_ipp(p).await,
    }
}
