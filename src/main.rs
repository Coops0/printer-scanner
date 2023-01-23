use anyhow::Result;
use clap::Parser;
use crate::scanner::scan_for_devices;

mod scanner;
mod identifier;

#[derive(Parser, Debug, Clone)]
#[command(long_about = None)]
pub struct Args {
    /// Amount of threads to simultaneously request on
    #[arg(short, long, default_value_t = 10)]
    threads: usize,

    /// Log failures as well
    #[arg(short, long)]
    verbose: bool,

    /// The subnet to generate ips for (use x to denote a wildcard)
    #[arg(short, long, default_value_t = String::from("10.208.x.x"))]
    ip_subnet: String,

    /// Display a progress bar
    #[arg(short, long)]
    progress_bar: bool,

    /// Print all ips to a file for debugging
    #[arg(short, long)]
    print_all: bool
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    scan_for_devices(args).await
    // for printer in printers {
    //     println!("{printer}{PRINTER_PAGE}");
    // }
}

fn subnet_generator(ip: String) -> Vec<String> {
    let mut current_passthrough = vec![];

    if !ip.contains('x') {
        return vec![ip];
    }

    for i in 1..=255 {
        current_passthrough.push(ip.replacen('x', i.to_string().as_str(), 1));
    }

    while current_passthrough[0].contains('x') {
        let mut temp = vec![];
        for p in current_passthrough {
            for i in 1..=255 {
                temp.push(p.replacen('x', i.to_string().as_str(), 1));
            }
        }

        current_passthrough = temp;
    }

    current_passthrough
}
