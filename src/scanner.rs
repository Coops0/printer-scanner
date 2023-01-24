use std::{
    fmt::{Display, Formatter},
    time::Duration,
};
use anyhow::{Result};
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use reqwest::{Client};
use thiserror::Error;
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    sync::{
        mpsc,
        mpsc::{UnboundedReceiver, UnboundedSender},
    },
    task::{
        self,
        JoinSet,
    },
};

use crate::{
    Args,
    subnet_generator,
    identifier::NetworkDevice,
};

pub async fn scan_for_devices(args: Args) -> Result<()> {
    // let net: Ipv4Net = args.ip_subnet.parse()?;
    let net = subnet_generator(args.ip_subnet.clone());

    let hosts = net//.hosts()
        .into_iter()
        .map(IpWrapper)
        .collect::<Vec<IpWrapper>>();

    if args.save_all {
        let _ = tokio::fs::remove_file("./all_ips.txt").await;

        let _ = File::create("./all_ips.txt")
            .await
            .unwrap()
            .write(
                hosts
                    .iter()
                    .map(|i| i.0.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_bytes()
            )
            .await?;

        println!("Printed all ips!");
    }

    let chunks = hosts.chunks(hosts.len() / args.threads)
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
    let mut progress_bar_task = None;
    if args.progress_bar {
        let (sender, receiver) = mpsc::unbounded_channel();
        progress_bar_scanner = Some(sender);

        progress_bar_task = Some(task::spawn(progress_bar_thread(hosts.len() as u64, receiver)));
    }

    let mut set = JoinSet::new();
    for servers in chunks {
        set.spawn(scanner_thread(servers.to_vec(), args.clone(), progress_bar_scanner.clone()));
    }

    let mut devices = vec![];
    while let Some(res) = set.join_next().await {
        if let Ok(r) = res.unwrap() {
            devices.push(r);
        }
    }

    if let Some(s) = progress_bar_scanner {
        let _ = s.send(ProgressBarMessage::Close);
        let _ = progress_bar_task.unwrap().await;
    }

    let devices = devices.concat();
    println!("-- Finished, found {} valid devices --", devices.len());

    let content = devices
        .iter()
        .map(|d| format!("{}:{}", d.0, d.1))
        .collect::<Vec<String>>()
        .join("\n");

    let _ = tokio::fs::remove_file("./devices.txt").await;

    let _ = File::create("./devices.txt")
        .await
        .unwrap()
        .write(content.as_bytes())
        .await?;

    println!("Successfully wrote to ./devices.txt");

    Ok(())
}

async fn scanner_thread(servers: Vec<IpWrapper>, args: Args, sender: Option<UnboundedSender<ProgressBarMessage>>) -> Result<Vec<(IpWrapper, NetworkDevice)>> {
    let mut client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(2))
        .build()?;

    let mut new_servers = vec![];

    for server in servers {
        let log = match scan(&mut client, &server).await {
            Ok(t) => {
                let m = format!("Valid device type of {t} on {}", server.url());
                new_servers.push((server.clone(), t.clone()));

                m
            }
            Err(e) => match e {
                ScanError::Timeout | ScanError::Connection => {
                    if args.verbose {
                        format!("{server} {e}")
                    } else {
                        String::new()
                    }
                }
                _ => format!("{server} {e}")
            }
        };

        if let Some(ref sender) = sender {
            if !log.is_empty() {
                let _ = sender.send(ProgressBarMessage::Message(log));
            }
            let _ = sender.send(ProgressBarMessage::Increment);
        }
    }

    Ok(new_servers)
}

async fn scan(client: &mut Client, server: &IpWrapper) -> Result<NetworkDevice, ScanError> {
    let res = client.get(server.url())
        .send()
        .await;

    let error = match res {
        Ok(o) => {
            let text = o.text().await.unwrap_or_default();
            return Ok(NetworkDevice::from_response(text));
        }
        Err(e) => e,
    };

    if error.is_timeout() {
        return Err(ScanError::Timeout);
    }

    if error.is_connect() {
        return Err(ScanError::Connection);
    }

    Err(ScanError::OtherError(error))
}

enum ProgressBarMessage {
    Increment,
    Message(String),
    Close,
}

async fn progress_bar_thread(amount: u64, mut rec: UnboundedReceiver<ProgressBarMessage>) {
    let pb = ProgressBar::new(amount);
    loop {
        match rec.recv().await {
            Some(m) => match m {
                ProgressBarMessage::Increment => pb.inc(1),
                ProgressBarMessage::Message(m) => pb.println(m),
                ProgressBarMessage::Close => {
                    pb.finish_with_message("prematurely done scanning");
                    return;
                }
            },
            None => return,
        }

        if amount <= pb.position() {
            pb.finish_with_message("finished sending requests");
            return;
        }
    }
}


#[derive(Clone)]
pub struct IpWrapper(pub String);

impl IpWrapper {
    pub fn url(&self) -> String {
        format!("https://{}", self.0)
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
    #[error("connection failed")]
    Connection,
    #[error("other weird web error {0:?}")]
    OtherError(reqwest::Error),
}