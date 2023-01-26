use std::env;
use std::env::args;
use std::process::ExitStatus;

use anyhow::Result;
use ipp::attribute::{IppAttribute, IppAttributeGroup};
use ipp::model::{DelimiterTag, IppVersion, Operation};
use ipp::payload::IppPayload;
use ipp::prelude::{AsyncIppClient, IppOperationBuilder, Uri};
use ipp::request::IppRequestResponse;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub async fn print(printer_system_name: &str, file_path: &str) -> Result<ExitStatus> {
    Ok(
        Command::new("lp")
            .arg("-d")
            .arg(printer_system_name)
            .arg(file_path)
            .spawn()?
            .wait()
            .await?
    )
}

pub async fn ipp_info(printer_ip: &str) -> Result<()> {
    let uri: Uri = Uri::try_from(printer_ip)?;
    let req = IppRequestResponse::new(
        IppVersion::v1_1(),
        Operation::GetPrinterAttributes,
        Some(uri.clone()),
    );

    let client = AsyncIppClient::new(uri);
    let resp = client.send(req).await?;
    if resp.header().status_code().is_success() {
        println!("{:?}", resp.attributes());
    }

    Ok(())
}

pub async fn print_ipp(printer_ip: &str, file_path: &str) -> Result<()> {
    let payload = IppPayload::new(std::fs::File::open(file_path)?);

    let uri: Uri = Uri::try_from(printer_ip)?;

    let mut builder = IppOperationBuilder::print_job(uri.clone(), payload)
        .user_name("noname")
        .job_title(printer_ip);

    let operation = builder.build();
    let client = AsyncIppClient::new(uri);
    let response = client.send(operation).await?;

    println!("IPP status code: {}", response.header().status_code());

    let attrs = response
        .attributes()
        .groups_of(DelimiterTag::JobAttributes)
        .flat_map(|g| g.attributes().values());

    for attr in attrs {
        println!("{}: {}", attr.name(), attr.value());
    }

    Ok(())
}