use std::time::Duration;
use reqwest::{Client, StatusCode};
use anyhow::Result;
use clap::Parser;
use tokio::task::JoinSet;

const PRINTER_PAGE: &str = "/hp/device/";

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    threads: usize,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Debug)]
enum ScanError {
    Timeout,
    NotOk(StatusCode),
    OtherError(reqwest::Error),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut server_range = vec![];
    for i in 1..255 {
        for ii in 1..255 {
            server_range.push(format!("10.208.{i}.{ii}"));
        }
    }

    let mut set = JoinSet::new();
    let chunks = server_range.chunks(args.threads);
    for servers in chunks {
        set.spawn(scanner_thread(servers.to_vec(), args.clone()));
    }

    let mut printers = vec![];
    while let Some(res) = set.join_next().await {
        if let Ok(r) = res.unwrap() {
            printers.push(r);
        }
    }

    println!("-- FINISHED --");
    // for printer in printers {
    //     println!("{printer}{PRINTER_PAGE}");
    // }
}


async fn scanner_thread(servers: Vec<String>, args: Args) -> Result<Vec<String>> {
    let mut client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut new_servers = vec![];

    for server in servers {
        match scan(&mut client, &server).await {
            Ok(_) => {
                println!("Valid printer page on https://{server}{PRINTER_PAGE}");
                new_servers.push(server);
            }
            Err(e) => match e {
                ScanError::Timeout => {
                    if args.verbose {
                        println!("Invalid printer {server}");
                    }
                }
                ScanError::NotOk(s) => {
                    println!("Weird status {s} on printer https://{server}{PRINTER_PAGE}");
                }
                ScanError::OtherError(e) => {
                    println!("Got other error {e:?} on printer https://{server}{PRINTER_PAGE}");
                }
            }
        }
    }

    Ok(new_servers)
}

async fn scan(client: &mut Client, server: &String) -> std::result::Result<(), ScanError> {
    let res = client.get(format!("http://{server}{PRINTER_PAGE}"))
        .send()
        .await;


    let error = match res {
        Ok(o) => {
            if o.status() == StatusCode::OK {
                return Ok(());
            }

            println!("non-successive code {} on https://{server}{PRINTER_PAGE}", o.status());
            return Err(ScanError::NotOk(o.status()));
        }
        Err(e) => e,
    };

    if error.is_timeout() {
        return Err(ScanError::Timeout);
    }

    println!("weird error on https://{server}{PRINTER_PAGE} -> {error:?}");
    Err(ScanError::OtherError(error))
}