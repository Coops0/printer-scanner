use anyhow::Result;
use ipp::{
    attribute::IppAttribute,
    model::DelimiterTag,
    payload::IppPayload,
    prelude::{AsyncIppClient, IppOperationBuilder, Uri},
    value::IppValue,
};

use crate::PrintArgs;

pub async fn print_ipp(args: PrintArgs) -> Result<()> {
    // todo fix this blocking call
    let payload = IppPayload::new(std::fs::File::open(args.file)?);

    let mut ip = args.ip.clone();
    if !ip.starts_with("http") {
        ip = format!("http://{ip}");
    }

    let uri: Uri = Uri::try_from(ip.clone())?;

    let mut builder = IppOperationBuilder::print_job(uri.clone(), payload)
        .user_name("noname")
        .job_title(ip);

    if args.copies != 1 {
        builder = builder.attribute(IppAttribute::new(
            "copies",
            IppValue::Integer(args.copies as i32),
        ));
    }

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
