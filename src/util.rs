use std::fmt::{Display, Formatter};
use thiserror::Error;

pub fn subnet_generator(ip: String) -> Vec<String> {
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
pub enum ScanError {
    #[error("timeout occurred after 10s")]
    Timeout,
    #[error("connection failed")]
    Connection,
    #[error("other weird web error {0:?}")]
    OtherError(reqwest::Error),
}
