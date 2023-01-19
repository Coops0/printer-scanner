extern crate alloc;

use anyhow::Result;
use clap::Parser;
use crate::scanner::scan_for_printers;

mod scanner;

#[derive(Parser, Debug, Clone)]
#[command(long_about = None)]
pub struct Args {
    /// Amount of threads to simultaneously request on
    #[arg(short, long, default_value_t = 10)]
    threads: usize,

    /// Log failures as well
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The subnet to generate ips for (use x to denote a wildcard)
    #[arg(short, long, default_value_t = String::from("10.208.0.0/24"))]
    ip_subnet: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    scan_for_printers(args).await
    // for printer in printers {
    //     println!("{printer}{PRINTER_PAGE}");
    // }
}