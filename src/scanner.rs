use std::time::Duration;

use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use reqwest::{redirect::Policy, Client};
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    sync::{mpsc, mpsc::UnboundedSender},
    task::{self, JoinSet},
};

use crate::{
    id::devices::NetworkDevice,
    threads,
    threads::{AppendMessage, ProgressBarMessage},
    util::{subnet_generator, IpWrapper, ScanError},
    ScannerArgs,
};

pub async fn scan_for_devices(args: ScannerArgs) -> Result<()> {
    let net = subnet_generator(args.ip_subnet.clone());

    let hosts = net.into_iter().map(IpWrapper).collect::<Vec<IpWrapper>>();

    if hosts.len() < args.threads {
        bail!("more threads than ips to scan");
    }

    let chunks = hosts
        .chunks(hosts.len() / args.threads)
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

        println!("10 randomly sampled generated IPs: {sample}");
    }

    let mut progress_bar = None;
    if args.progress_bar {
        let (sender, receiver) = mpsc::unbounded_channel();

        progress_bar = Some((
            sender,
            task::spawn(threads::progress_bar_thread(hosts.len() as u64, receiver)),
        ));
    }

    let mut appender = None;
    if args.append_file {
        let (sender, receiver) = mpsc::unbounded_channel();

        appender = Some((sender, task::spawn(threads::append_thread(receiver))));
    }

    let mut set = JoinSet::new();
    for servers in chunks {
        let mut pb = None;
        if let Some((s, _)) = &progress_bar {
            pb = Some(s.clone());
        }

        let mut ap = None;
        if let Some((a, _)) = &appender {
            ap = Some(a.clone());
        }

        set.spawn(scanner_thread(servers.to_vec(), args.clone(), pb, ap));
    }

    let mut devices = vec![];
    while let Some(res) = set.join_next().await {
        if let Ok(r) = res.unwrap() {
            devices.push(r);
        }
    }

    if let Some((sender, t)) = progress_bar {
        let _ = sender.send(ProgressBarMessage::Close);
        let _ = t.await;
    }

    if let Some((sender, t)) = appender {
        let _ = sender.send(AppendMessage::Close);
        let _ = t.await;
    }

    let devices = devices.concat();
    println!("-- Finished, found {} valid devices --", devices.len());

    if !args.append_file {
        let content = devices
            .iter()
            .map(|d| format!("{}:{}", d.0, d.1))
            .collect::<Vec<String>>()
            .join("\n");

        let _ = tokio::fs::remove_file("./devices.txt").await;

        let _ = File::create("./devices.txt")
            .await?
            .write(content.as_bytes())
            .await?;

        println!("Successfully wrote to ./devices.txt");
    }

    Ok(())
}

async fn scanner_thread(
    servers: Vec<IpWrapper>,
    args: ScannerArgs,
    sender: Option<UnboundedSender<ProgressBarMessage>>,
    appender: Option<UnboundedSender<AppendMessage>>,
) -> Result<Vec<(IpWrapper, NetworkDevice)>> {
    let mut client = Client::builder()
        .danger_accept_invalid_certs(true)
        .redirect(Policy::none())
        .timeout(Duration::from_millis(args.timeout))
        .build()?;

    let mut new_servers = vec![];

    for server in servers {
        let log = match scan(&mut client, &server).await {
            Ok(t) => {
                let m = format!("Valid device type of {t} on {}", server.url());
                if let Some(ref a) = appender {
                    let _ = a.send(AppendMessage::Amendment(format!("{}:{t}\n", server.0)));
                }

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
                ScanError::OtherError(_) => format!("{server} {e}"),
            },
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

async fn scan(client: &mut Client, ip: &IpWrapper) -> Result<NetworkDevice, ScanError> {
    let res = client.get(ip.url()).send().await;

    let error = match res {
        Ok(o) => {
            let text = o.text().await.unwrap_or_default();
            return Ok(NetworkDevice::from_response(ip, text));
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