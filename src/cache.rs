use std::collections::HashMap;

use anyhow::{bail, Context};
use ipp::model::DelimiterTag;
use ipp::prelude::{AsyncIppClient, IppAttribute, IppAttributeGroup, IppOperationBuilder, Uri};

use crate::id::devices::{NetworkDevice, Printer};
use crate::util::IpWrapper;

pub struct CachedPrinter {
    pub ip: IpWrapper,
    pub model: Option<Printer>,
    pub attributes: Option<HashMap<String, IppAttribute>>,
}

impl CachedPrinter {
    pub fn new(ip: IpWrapper, model: Option<Printer>) -> Self {
        Self {
            ip,
            model,
            attributes: None
        }
    }

    pub fn uri(&self) -> Option<Uri> {
        format!("http://{}", self.ip.0).parse().ok()
    }

    pub async fn cache(&mut self) -> anyhow::Result<()> {
        let uri = self.uri().context("bad uri")?;

        let operation = IppOperationBuilder::get_printer_attributes(uri.clone()).build();
        let client = AsyncIppClient::new(uri);
        let resp = client.send(operation).await?;

        if !resp.header().status_code().is_success() {
            bail!("response status is not success")
        }

        let attributes = resp
            .attributes()
            .groups_of(DelimiterTag::PrinterAttributes)
            .next()
            .context("no next of printer attributes")?
            .attributes()
            .clone();

         self.attributes = Some(attributes);
        Ok(())
    }
}
