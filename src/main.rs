use std::time::Duration;
use reqwest::{Client, StatusCode};
use anyhow::Result;
use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinSet;

const PRINTER_PAGE: &str = "/hp/device/";

#[derive(Parser, Debug, Clone)]
#[command(long_about = None)]
struct Args {
    /// Amount of threads to simultaneously request on
    #[arg(short, long, default_value_t = 10)]
    threads: usize,

    /// Log failures as well
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The subnet to generate ips for (use x to denote a wildcard)
    #[arg(short, long, default_value_t = String::from("10.208.x.x"))]
    ip_subnet: String,
}

#[derive(Debug)]
enum ScanError {
    Timeout,
    Connection,
    NotOk(StatusCode),
    OtherError(reqwest::Error),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let server_range = {
        let mut server_range: Vec<String> = vec![];
        let mut ip_sample = args.ip_subnet.clone();
        loop {
            if !ip_sample.contains("x") {
                server_range.retain(|s| !s.contains("x"));
                break;
            }


            let mut current_range = server_range.clone();
            if current_range.is_empty() {
                current_range.push(ip_sample.clone());
            }

            for ip in current_range {
                for i in 1..255 {
                    server_range.push(ip.replacen("x", &i.to_string(), 1));
                }
            }

            ip_sample = server_range.last().unwrap().clone();
        }

        server_range
    };

    println!("Parsed {} ips to scan, start using {} threads", server_range.len(), args.threads);

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

    let printers = printers.concat();

    println!("-- Finished, found {} valid printers --", printers.len());

    let content = printers.join("\n");


    let _ = tokio::fs::remove_file("./printers.txt").await;

    File::create("./printers.txt")
        .await
        .unwrap()
        .write(content.as_bytes())
        .await
        .unwrap();

    println!("Successfully wrote to ./printers.txt");
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
                ScanError::Connection => {
                    if args.verbose {
                        println!("Connection error {server}");
                    }
                }
                ScanError::NotOk(s) => println!("Weird status {s} on printer https://{server}{PRINTER_PAGE}"),
                ScanError::OtherError(e) => println!("Got other error {e:?} on printer https://{server}{PRINTER_PAGE}")
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

            // println!("non-successive code {} on https://{server}{PRINTER_PAGE}", o.status());
            return Err(ScanError::NotOk(o.status()));
        }
        Err(e) => e,
    };

    if error.is_timeout() {
        return Err(ScanError::Timeout);
    }

    if error.is_connect() {
        return Err(ScanError::Connection);
    }

    println!("weird error on https://{server}{PRINTER_PAGE} -> {error:?}");
    Err(ScanError::OtherError(error))
}