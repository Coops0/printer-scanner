use std::{
    fmt::{Display, Formatter},
    time::Duration
};
use anyhow::Result;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use reqwest::{Client, StatusCode};
use thiserror::Error;
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    sync::{
        mpsc,
        mpsc::{UnboundedReceiver, UnboundedSender}
    },
    task::{
        self,
        JoinSet
    }
};

use crate::{Args, subnet_generator};

pub const PRINTER_PAGE: &str = "/hp/device/";

pub async fn scan_for_printers(args: Args) -> Result<()> {
    // let net: Ipv4Net = args.ip_subnet.parse()?;
    let net = subnet_generator(args.ip_subnet.clone());

    let hosts = net//.hosts()
        .into_iter()
        .map(IpWrapper)
        .collect::<Vec<IpWrapper>>();

    let chunks = hosts.chunks(args.threads)
        .collect::<Vec<&[IpWrapper]>>();

    let avg = {
        let chunks_len = chunks.iter().map(|a| a.len());
        let l = chunks_len.len();

        chunks_len.sum::<usize>() / l
    };

    println!("Parsed {} ips to scan, start using {} threads chunked into arrays with an avg length of {avg}", hosts.len(), args.threads);

    if hosts.len() > 10 {
        let sample = hosts
            .choose_multiple(&mut rand::thread_rng(), 10)
            .map(|i| i.0.clone())
            .collect::<Vec<String>>()
            .join(", ");

        println!("10 randomly sampled generated IPs: {sample}")
    }

    let mut progress_bar_scanner = None;
    if args.progress_bar {
        let (sender, receiver) = mpsc::unbounded_channel();
        progress_bar_scanner = Some(sender);

        task::spawn(progress_bar_task(hosts.len() as u64, receiver));
    }

    println!("Spawning threads...");
    let mut set = JoinSet::new();
    for servers in chunks {
        set.spawn(scanner_thread(servers.to_vec(), args.clone(), progress_bar_scanner.clone()));
    }

    println!("Spawned all threads!");
    let mut printers = vec![];
    while let Some(res) = set.join_next().await {
        if let Ok(r) = res.unwrap() {
            printers.push(r);
        }
    }

    let printers = printers.concat();

    println!("-- Finished, found {} valid printers --", printers.len());

    let content = printers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n");

    let _ = tokio::fs::remove_file("./printers.txt").await;

    let _ = File::create("./printers.txt")
        .await
        .unwrap()
        .write(content.as_bytes())
        .await?;

    println!("Successfully wrote to ./printers.txt");

    Ok(())
}

async fn scanner_thread(servers: Vec<IpWrapper>, args: Args, sender: Option<UnboundedSender<()>>) -> Result<Vec<IpWrapper>> {
    let mut client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut new_servers = vec![];

    for server in servers {
        match scan(&mut client, &server).await {
            Ok(_) => {
                println!("Valid printer page on {}", server.url());
                new_servers.push(server);
            }
            Err(e) => match e {
                ScanError::Timeout | ScanError::Connection => if args.verbose {
                    println!("{server} {e}");
                },
                _ => println!("{server} {e}")
            }
        }

        if let Some(ref sender) = sender {
            let _ = sender.send(());
        }
    }

    Ok(new_servers)
}

async fn scan(client: &mut Client, server: &IpWrapper) -> Result<(), ScanError> {
    let res = client.get(server.url())
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

    // println!("weird error on {} -> {error:?}", server.url());
    Err(ScanError::OtherError(error))
}

async fn progress_bar_task(amount: u64, mut rec: UnboundedReceiver<()>) {
    let pb = ProgressBar::new(amount);
    loop {
        match rec.recv().await {
            Some(_) => pb.inc(1),
            None => pb.abandon_with_message("channel was closed unexpectedly")
        }

        if amount <= pb.position() {
            pb.finish_with_message("finished sending requests");
        }
    }
}

#[derive(Clone)]
pub struct IpWrapper(pub String);

impl IpWrapper {
    pub fn url(&self) -> String {
        format!("https://{}{PRINTER_PAGE}", self.0)
    }
}

impl Display for IpWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Error, Debug)]
enum ScanError {
    #[error("timeout occurred after 10s")]
    Timeout,
    #[error("connection failed / usually bad host")]
    Connection,
    #[error("status code was not 200, was instead {0:?}")]
    NotOk(StatusCode),
    #[error("other weird web error {0:?}")]
    OtherError(reqwest::Error),
}