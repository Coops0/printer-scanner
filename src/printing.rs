use std::env::args;

use anyhow::{bail, Context, Result};
use ipp::{
    attribute::IppAttribute,
    model::DelimiterTag,
    model::StatusCode,
    payload::IppPayload,
    prelude::{AsyncIppClient, IppOperationBuilder, Uri},
    value::IppValue,
};
use ipp::prelude::IppRequestResponse;
use tokio::fs::File;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::PrintArgs;
use crate::printer::CachedPrinter;

// PNG, HEIC, and TIFF print garbage spam
// jpg/jpeg says unsupported
// below extensions are manually checked and verified to work
// I would convert any image to a pdf
const WHITELISTED_EXT: &[&str] = &[
    ".docx",
    ".pdf",
    ".txt"
];

pub async fn print_ipp(args: PrintArgs) -> Result<()> {
    let mut ip = args.ip.clone();
    if ip.starts_with("http") {
        ip = ip.replace("http://", "");
        ip = ip.replace("https://", "");
    }

    let f = args.file.to_lowercase();

    let mut printer = CachedPrinter::new(ip, None);
    if args.identify_formats {
        println!("Identifying printer formats...");
        printer.fetch_attributes().await.context("fetch attributes failed")?;

        let exts = printer.supported_extensions.as_ref().unwrap();
        println!("The printer supports the types -> {exts:?}");

        if args.only_detect_formats {
            return Ok(());
        }
    }

    if !WHITELISTED_EXT.iter().any(|e| f.ends_with(e)) {
        if args.bypass_ext {
            println!("Bypassing invalid file extension...");
        } else {
            bail!("not whitelisted file ext");
        }
    }

    let payload = IppPayload::new_async(File::open(args.file).await?.compat());
    let response = print(&printer, payload, args.copies as i32).await.context("failed to print")?;

    println!("IPP status code: {}", response.header().status_code());

    let attrs = response
        .attributes()
        .groups_of(DelimiterTag::JobAttributes)
        .flat_map(|g| g.attributes().values());

    for attr in attrs {
        println!("{}: {}", attr.name(), attr.value());
    }

    if response.header().status_code() != StatusCode::SuccessfulOk {
        bail!("non ok status code")
    }

    Ok(())
}


pub async fn print(printer: &CachedPrinter, payload: IppPayload, copies: i32) -> Result<IppRequestResponse> {
    let uri = printer.uri().context("failed to gen uri")?;
    let mut builder = IppOperationBuilder::print_job(uri.clone(), payload)
        .user_name("noname")
        .job_title(&printer.ip);

    if copies != 1 {
        builder = builder.attribute(IppAttribute::new(
            "copies",
            IppValue::Integer(copies),
        ));
    }

    let operation = builder.build();
    let client = AsyncIppClient::new(uri);

    let response = client.send(operation).await?;
    Ok(response)
}