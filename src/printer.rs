use std::collections::HashMap;

use anyhow::{bail, Context};
use ipp::model::DelimiterTag;
use ipp::prelude::{AsyncIppClient, IppAttribute, IppOperationBuilder, Uri};

use anyhow::Result;

use crate::id::devices::Printer;


pub struct CachedPrinter {
    pub ip: String,
    pub model: Option<Printer>,
    pub attributes: Option<HashMap<String, IppAttribute>>,
    pub supported_extensions: Option<Vec<String>>,
}

impl CachedPrinter {
    pub fn new(ip: String, model: Option<Printer>) -> Self {
        Self {
            ip,
            model,
            attributes: None,
            supported_extensions: None,
        }
    }

    pub fn uri(&self) -> Option<Uri> {
        format!("http://{}", self.ip).parse().ok()
    }

    pub async fn fetch_attributes(&mut self) -> Result<()> {
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

        if let Some(supported) = &attributes.get(IppAttribute::DOCUMENT_FORMAT_SUPPORTED) {
            self.supported_extensions = supported.value()
                .as_array()
                .context("document format supported not in array format")?
                .iter()
                .map(|v| v
                    .as_mime_media_type()
                    .context("failed to convert value to mime media type")
                    .map(String::clone)
                )
                .collect::<Result<Vec<String>>>()
                .ok();
        }

        self.attributes = Some(attributes);
        Ok(())
    }
}
